use super::http::{send_with_resilience, SourceClass, HTTP_CLIENT};
use crate::models::gdacs::GdacsAlert;
use quick_xml::escape::unescape;
use quick_xml::events::Event;
use quick_xml::Reader;

const GDACS_RSS_URL: &str = "https://www.gdacs.org/xml/rss.xml";

pub async fn fetch_gdacs_alerts() -> Result<Vec<GdacsAlert>, String> {
    let response =
        send_with_resilience("gdacs", SourceClass::Standard, "GDACS RSS request", || {
            HTTP_CLIENT.get(GDACS_RSS_URL)
        })
        .await?;

    let text = response
        .text()
        .await
        .map_err(|_| "Failed to read GDACS response".to_string())?;

    parse_gdacs_rss(&text)
}

fn parse_gdacs_rss(xml: &str) -> Result<Vec<GdacsAlert>, String> {
    let mut reader = Reader::from_str(xml);
    let mut alerts = Vec::new();
    let mut buf = Vec::new();

    let mut in_item = false;
    let mut current_tag = String::new();

    // Item fields
    let mut title = String::new();
    let mut description = String::new();
    let mut link = String::new();
    let mut pub_date = String::new();
    let mut alert_level = String::new();
    let mut event_type = String::new();
    let mut country = String::new();
    let mut lat: Option<f64> = None;
    let mut lon: Option<f64> = None;
    let mut event_id = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                current_tag = name.clone();

                if name == "item" {
                    in_item = true;
                    title.clear();
                    description.clear();
                    link.clear();
                    pub_date.clear();
                    alert_level.clear();
                    event_type.clear();
                    country.clear();
                    lat = None;
                    lon = None;
                    event_id.clear();
                }
            }
            Ok(Event::Empty(_)) => {}
            Ok(Event::Text(ref e)) => {
                if !in_item {
                    buf.clear();
                    continue;
                }

                let text = e
                    .decode()
                    .ok()
                    .and_then(|text| unescape(&text).ok().map(|text| text.into_owned()))
                    .unwrap_or_default();
                let text = text.trim().to_string();
                if text.is_empty() {
                    buf.clear();
                    continue;
                }

                match current_tag.as_str() {
                    "title" => title = text,
                    "description" => description = text,
                    "link" => link = text,
                    "pubDate" => pub_date = text,
                    "gdacs:alertlevel" => alert_level = text,
                    "gdacs:eventtype" => event_type = text,
                    "gdacs:country" => country = text,
                    "gdacs:eventid" => event_id = text,
                    "geo:lat" => lat = text.parse().ok(),
                    "geo:long" => lon = text.parse().ok(),
                    "georss:point" => {
                        // Format: "lat lon"
                        let parts: Vec<&str> = text.split_whitespace().collect();
                        if parts.len() == 2 {
                            lat = parts[0].parse().ok();
                            lon = parts[1].parse().ok();
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::CData(ref e)) => {
                if !in_item {
                    buf.clear();
                    continue;
                }
                let text = String::from_utf8_lossy(e.as_ref()).to_string();
                let text = text.trim().to_string();
                if text.is_empty() {
                    buf.clear();
                    continue;
                }
                if current_tag == "description" {
                    description = text;
                }
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name == "item" && in_item {
                    in_item = false;

                    if let (Some(latitude), Some(longitude)) = (lat, lon) {
                        if !latitude.is_finite() || !longitude.is_finite() {
                            buf.clear();
                            continue;
                        }
                        let id = if event_id.is_empty() {
                            // Use lat/lon + pub_date for uniqueness when event_id is missing
                            let date_slug: String =
                                pub_date.chars().filter(|c| c.is_alphanumeric()).collect();
                            format!("gdacs-{event_type}-{latitude:.4}-{longitude:.4}-{date_slug}")
                        } else {
                            format!("gdacs-{event_type}-{event_id}")
                        };

                        alerts.push(GdacsAlert {
                            id,
                            title: title.clone(),
                            description: strip_html(&description),
                            alert_type: event_type.clone(),
                            severity: alert_level.clone(),
                            latitude,
                            longitude,
                            pub_date: pub_date.clone(),
                            link: link.clone(),
                            country: country.clone(),
                        });
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {e}")),
            _ => {}
        }
        buf.clear();
    }

    Ok(alerts)
}

fn strip_html(s: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }
    // Collapse whitespace
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::parse_gdacs_rss;

    #[test]
    fn parses_minimal_gdacs_item() {
        let xml = r#"
        <rss version="2.0" xmlns:gdacs="http://www.gdacs.org" xmlns:geo="http://www.w3.org/2003/01/geo/wgs84_pos#">
          <channel>
            <item>
              <title>Flood in Exampleland</title>
              <description><![CDATA[<p>Heavy flood warning</p>]]></description>
              <link>https://www.gdacs.org/report.aspx?eventid=123</link>
              <pubDate>Mon, 01 Mar 2026 12:00:00 GMT</pubDate>
              <gdacs:alertlevel>Orange</gdacs:alertlevel>
              <gdacs:eventtype>FL</gdacs:eventtype>
              <gdacs:country>Exampleland</gdacs:country>
              <gdacs:eventid>123</gdacs:eventid>
              <geo:lat>10.5</geo:lat>
              <geo:long>20.5</geo:long>
            </item>
          </channel>
        </rss>
        "#;

        let alerts = parse_gdacs_rss(xml).expect("xml should parse");
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].alert_type, "FL");
        assert_eq!(alerts[0].severity, "Orange");
        assert!(alerts[0].description.contains("Heavy flood warning"));
    }
}
