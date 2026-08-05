use super::http::{send_with_resilience, SourceClass, HTTP_CLIENT};
use crate::models::volcano::Volcano;
use quick_xml::escape::unescape;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::BTreeMap;

const SMITHSONIAN_WEEKLY_RSS_URL: &str = "https://volcano.si.edu/news/WeeklyVolcanoRSS.xml";

pub async fn get_active_volcanoes() -> Result<Vec<Volcano>, String> {
    let response = send_with_resilience(
        "volcanoes",
        SourceClass::Standard,
        "Smithsonian volcano feed request",
        || HTTP_CLIENT.get(SMITHSONIAN_WEEKLY_RSS_URL),
    )
    .await?;

    let xml = response
        .text()
        .await
        .map_err(|_| "Failed to read Smithsonian volcano feed".to_string())?;

    parse_weekly_report_rss(&xml)
}

pub fn fallback_volcanoes() -> Vec<Volcano> {
    vec![
        Volcano {
            id: "kilauea".into(),
            name: "Kilauea".into(),
            latitude: 19.421,
            longitude: -155.287,
            status: "warning".into(),
            last_eruption: "2023-ongoing".into(),
            description: "Shield volcano, Hawaii".into(),
        },
        Volcano {
            id: "etna".into(),
            name: "Mount Etna".into(),
            latitude: 37.748,
            longitude: 14.999,
            status: "watch".into(),
            last_eruption: "2024".into(),
            description: "Stratovolcano, Sicily, Italy".into(),
        },
        Volcano {
            id: "stromboli".into(),
            name: "Stromboli".into(),
            latitude: 38.789,
            longitude: 15.213,
            status: "advisory".into(),
            last_eruption: "2024".into(),
            description: "Stratovolcano, Aeolian Islands, Italy".into(),
        },
        Volcano {
            id: "piton".into(),
            name: "Piton de la Fournaise".into(),
            latitude: -21.244,
            longitude: 55.708,
            status: "watch".into(),
            last_eruption: "2024".into(),
            description: "Shield volcano, Réunion Island".into(),
        },
        Volcano {
            id: "sakurajima".into(),
            name: "Sakurajima".into(),
            latitude: 31.585,
            longitude: 130.657,
            status: "warning".into(),
            last_eruption: "2024-ongoing".into(),
            description: "Stratovolcano, Kyushu, Japan".into(),
        },
        Volcano {
            id: "popocatepetl".into(),
            name: "Popocatépetl".into(),
            latitude: 19.023,
            longitude: -98.622,
            status: "warning".into(),
            last_eruption: "2024-ongoing".into(),
            description: "Stratovolcano, Mexico".into(),
        },
        Volcano {
            id: "merapi".into(),
            name: "Mount Merapi".into(),
            latitude: -7.541,
            longitude: 110.446,
            status: "watch".into(),
            last_eruption: "2024".into(),
            description: "Stratovolcano, Java, Indonesia".into(),
        },
        Volcano {
            id: "semeru".into(),
            name: "Semeru".into(),
            latitude: -8.108,
            longitude: 112.922,
            status: "watch".into(),
            last_eruption: "2024".into(),
            description: "Stratovolcano, Java, Indonesia".into(),
        },
        Volcano {
            id: "erebus".into(),
            name: "Mount Erebus".into(),
            latitude: -77.53,
            longitude: 167.17,
            status: "normal".into(),
            last_eruption: "Continuous".into(),
            description: "Stratovolcano, Antarctica".into(),
        },
        Volcano {
            id: "fuego".into(),
            name: "Volcán de Fuego".into(),
            latitude: 14.473,
            longitude: -90.88,
            status: "warning".into(),
            last_eruption: "2024-ongoing".into(),
            description: "Stratovolcano, Guatemala".into(),
        },
        Volcano {
            id: "taal".into(),
            name: "Taal".into(),
            latitude: 14.002,
            longitude: 120.993,
            status: "advisory".into(),
            last_eruption: "2022".into(),
            description: "Caldera, Luzon, Philippines".into(),
        },
        Volcano {
            id: "villarrica".into(),
            name: "Villarrica".into(),
            latitude: -39.42,
            longitude: -71.93,
            status: "advisory".into(),
            last_eruption: "2024".into(),
            description: "Stratovolcano, Chile".into(),
        },
        Volcano {
            id: "mauna_loa".into(),
            name: "Mauna Loa".into(),
            latitude: 19.475,
            longitude: -155.608,
            status: "normal".into(),
            last_eruption: "2022".into(),
            description: "Shield volcano, Hawaii".into(),
        },
        Volcano {
            id: "aso".into(),
            name: "Mount Aso".into(),
            latitude: 32.884,
            longitude: 131.104,
            status: "advisory".into(),
            last_eruption: "2024".into(),
            description: "Caldera, Kyushu, Japan".into(),
        },
        Volcano {
            id: "ruang".into(),
            name: "Ruang".into(),
            latitude: 2.30,
            longitude: 125.37,
            status: "warning".into(),
            last_eruption: "2024".into(),
            description: "Stratovolcano, Sulawesi, Indonesia".into(),
        },
    ]
}

fn parse_weekly_report_rss(xml: &str) -> Result<Vec<Volcano>, String> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();

    let mut in_item = false;
    // One accumulator per element. quick-xml 0.38+ splits an element's character data
    // across several events: Text, CData, and GeneralRef for each `&entity;`. Assigning
    // per-event keeps only the last fragment, so accumulate here and commit on End.
    let mut current_text = String::new();

    let mut title = String::new();
    let mut description = String::new();
    let mut pub_date = String::new();
    let mut guid = String::new();
    let mut lat: Option<f64> = None;
    let mut lon: Option<f64> = None;

    let mut by_id: BTreeMap<String, Volcano> = BTreeMap::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                current_text.clear();
                if name == "item" {
                    in_item = true;
                    title.clear();
                    description.clear();
                    pub_date.clear();
                    guid.clear();
                    lat = None;
                    lon = None;
                }
            }
            Ok(Event::Text(ref e)) => {
                if !in_item {
                    buf.clear();
                    continue;
                }

                // From 0.38 on, Text carries no escaped parts; entities arrive as GeneralRef.
                if let Ok(text) = e.decode() {
                    current_text.push_str(&text);
                }
            }
            Ok(Event::CData(ref e)) => {
                if !in_item {
                    buf.clear();
                    continue;
                }

                current_text.push_str(&String::from_utf8_lossy(e.as_ref()));
            }
            Ok(Event::GeneralRef(ref e)) => {
                if !in_item {
                    buf.clear();
                    continue;
                }

                match e.resolve_char_ref() {
                    // Numeric form: &#38; or &#x26;
                    Ok(Some(ch)) => current_text.push(ch),
                    // Named form. unescape knows the five XML predefined entities; an
                    // unrecognised entity is kept as written rather than dropped, so a
                    // parse failure can never silently shorten a field.
                    _ => {
                        let name = e.decode().map(|n| n.into_owned()).unwrap_or_default();
                        let raw = format!("&{name};");
                        match unescape(&raw) {
                            Ok(resolved) => current_text.push_str(&resolved),
                            Err(_) => current_text.push_str(&raw),
                        }
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                // Commit the accumulated character data. The closing tag names the field
                // unambiguously, so this replaces the old per-event assignment keyed on
                // current_tag, which lost every fragment but the last.
                if in_item {
                    let trimmed = current_text.trim();
                    if !trimmed.is_empty() {
                        match name.as_str() {
                            "title" => title = trimmed.to_string(),
                            "description" => description = trimmed.to_string(),
                            "pubDate" => pub_date = trimmed.to_string(),
                            "guid" => guid = trimmed.to_string(),
                            "georss:point" => {
                                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                                if parts.len() == 2 {
                                    lat = parts[0].parse::<f64>().ok();
                                    lon = parts[1].parse::<f64>().ok();
                                }
                            }
                            _ => {}
                        }
                    }
                }
                current_text.clear();

                if name == "item" && in_item {
                    in_item = false;

                    let Some(latitude) = lat else {
                        buf.clear();
                        continue;
                    };
                    let Some(longitude) = lon else {
                        buf.clear();
                        continue;
                    };
                    if !latitude.is_finite() || !longitude.is_finite() {
                        buf.clear();
                        continue;
                    }

                    let volcano_name = parse_volcano_name(&title);
                    let status = infer_status(&title, &description);
                    let eruption_date = normalize_date(&pub_date);
                    let id = infer_id(&guid, &volcano_name, &eruption_date);

                    let volcano = Volcano {
                        id: id.clone(),
                        name: volcano_name.clone(),
                        latitude,
                        longitude,
                        status: status.to_string(),
                        last_eruption: eruption_date,
                        description: summarize_description(&description),
                    };
                    by_id.insert(id, volcano);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Failed to parse Smithsonian volcano feed: {e}")),
            _ => {}
        }
        buf.clear();
    }

    let mut result: Vec<Volcano> = by_id.into_values().collect();
    if result.is_empty() {
        return Err("Smithsonian feed did not contain parseable volcano entries".to_string());
    }
    result.sort_by(|a, b| {
        status_rank(&a.status)
            .cmp(&status_rank(&b.status))
            .then_with(|| a.name.cmp(&b.name))
    });
    Ok(result)
}

fn parse_volcano_name(title: &str) -> String {
    let pre = title.split(" - Report for").next().unwrap_or(title).trim();
    let name = pre.split(" (").next().unwrap_or(pre).trim();
    if name.is_empty() {
        "Unknown volcano".to_string()
    } else {
        name.to_string()
    }
}

fn infer_status(title: &str, description: &str) -> &'static str {
    let lower = format!("{} {}", title.to_lowercase(), description.to_lowercase());
    if lower.contains("new eruptive activity") || lower.contains("eruption") {
        "warning"
    } else if lower.contains("continuing activity") || lower.contains("ash plume") {
        "watch"
    } else {
        "advisory"
    }
}

fn normalize_date(pub_date: &str) -> String {
    chrono::DateTime::parse_from_rfc2822(pub_date)
        .map(|d| d.date_naive().to_string())
        .unwrap_or_else(|_| pub_date.to_string())
}

fn infer_id(guid: &str, name: &str, date: &str) -> String {
    if let Some(fragment) = guid.split('#').next_back() {
        if !fragment.is_empty() && fragment != guid {
            return fragment.to_string();
        }
    }
    let slug = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    format!("{slug}-{date}")
}

fn summarize_description(raw: &str) -> String {
    let stripped = strip_html(raw);
    let first_sentence = stripped
        .split(". ")
        .next()
        .unwrap_or(stripped.as_str())
        .trim()
        .to_string();
    if first_sentence.len() > 180 {
        format!("{}...", &first_sentence[..177])
    } else {
        first_sentence
    }
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

    result
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn status_rank(status: &str) -> u8 {
    match status {
        "warning" => 0,
        "watch" => 1,
        "advisory" => 2,
        "normal" => 3,
        _ => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::parse_weekly_report_rss;

    #[test]
    fn parses_rss_items_into_volcano_entries() {
        let xml = r#"
        <rss version="2.0" xmlns:georss="http://www.georss.org/georss">
          <channel>
            <item>
              <title>Fuego - Report for 2026-02-25</title>
              <description><![CDATA[<p>New eruptive activity with ash plume.</p>]]></description>
              <pubDate>Wed, 25 Feb 2026 00:00:00 +0000</pubDate>
              <guid>https://example.com/#fuego-2026-02-25</guid>
              <georss:point>14.473 -90.88</georss:point>
            </item>
          </channel>
        </rss>
        "#;

        let volcanoes = parse_weekly_report_rss(xml).expect("rss should parse");
        assert_eq!(volcanoes.len(), 1);
        assert_eq!(volcanoes[0].name, "Fuego");
        assert_eq!(volcanoes[0].status, "warning");
        assert_eq!(volcanoes[0].id, "fuego-2026-02-25");
        assert_eq!(volcanoes[0].last_eruption, "2026-02-25");
    }

    // Same defect as gdacs.rs: quick-xml 0.38+ reports `&amp;` as a separate
    // Event::GeneralRef, so a parser that assigns per Text event keeps only the
    // fragment after the last entity. Without this the volcano name silently
    // truncates to whatever follows the ampersand.
    #[test]
    fn preserves_entity_refs_in_item_fields() {
        let xml = r#"
        <rss version="2.0" xmlns:georss="http://www.georss.org/georss">
          <channel>
            <item>
              <title>Villarrica &amp; Llaima - Report for 2026-02-25</title>
              <description>Ash &amp; steam plume observed</description>
              <pubDate>Wed, 25 Feb 2026 00:00:00 +0000</pubDate>
              <guid>https://example.com/#villarrica-2026-02-25</guid>
              <georss:point>-39.42 -71.93</georss:point>
            </item>
          </channel>
        </rss>
        "#;

        let volcanoes = parse_weekly_report_rss(xml).expect("rss should parse");
        assert_eq!(volcanoes.len(), 1);
        assert_eq!(volcanoes[0].name, "Villarrica & Llaima");
        assert!(
            volcanoes[0].description.contains("Ash & steam"),
            "description lost entity text: {:?}",
            volcanoes[0].description
        );
    }

    #[test]
    fn fails_when_no_parseable_entries_exist() {
        let xml = r#"<rss version="2.0"><channel><item><title>Missing coordinates</title></item></channel></rss>"#;
        let err = parse_weekly_report_rss(xml).expect_err("should fail without coordinates");
        assert!(err.contains("parseable volcano entries"));
    }
}
