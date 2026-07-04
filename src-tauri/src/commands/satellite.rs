use crate::calculations::orbit;
use crate::db::Database;
use crate::fetchers::tle;
use crate::models::satellite::{PassPrediction, SatelliteData};
use tauri::State;

const SATELLITES: &[(&str, &str)] = &[
    (
        "25544",
        "https://celestrak.org/NORAD/elements/gp.php?CATNR=25544&FORMAT=TLE",
    ),
    (
        "20580",
        "https://celestrak.org/NORAD/elements/gp.php?CATNR=20580&FORMAT=TLE",
    ),
    (
        "48274",
        "https://celestrak.org/NORAD/elements/gp.php?CATNR=48274&FORMAT=TLE",
    ),
];

async fn get_tle_cached(
    db: &Database,
    cat_nr: &str,
    url: &str,
) -> Result<Vec<tle::TlePair>, String> {
    let cache_key = format!("tle:{cat_nr}");

    // Try cache (6 hour TTL)
    if let Some(cached) = db.get_cached_response(&cache_key, 21600) {
        let pairs = parse_cached_tle(&cached);
        if !pairs.is_empty() {
            return Ok(pairs);
        }
    }

    let pairs = tle::fetch_tle(url).await?;

    // Cache the raw TLE text
    let cache_text: String = pairs
        .iter()
        .map(|p| format!("{}\n{}\n{}", p.name, p.line1, p.line2))
        .collect::<Vec<_>>()
        .join("\n");
    db.set_cached_response(&cache_key, &cache_text);

    Ok(pairs)
}

fn parse_cached_tle(text: &str) -> Vec<tle::TlePair> {
    tle::parse_tle_text(text).unwrap_or_default()
}

pub async fn get_satellite_positions_inner(db: &Database) -> Result<SatelliteData, String> {
    let now = chrono::Utc::now().timestamp();
    let mut positions = Vec::new();
    let mut orbits = Vec::new();

    for (cat_nr, url) in SATELLITES {
        match get_tle_cached(db, cat_nr, url).await {
            Ok(pairs) => {
                for pair in &pairs {
                    let id = format!("sat-{cat_nr}");

                    if let Some(pos) =
                        orbit::propagate_position(&id, &pair.name, &pair.line1, &pair.line2, now)
                    {
                        positions.push(pos);
                    }

                    // ~92 min orbit for LEO
                    if let Some(track) = orbit::predict_orbit_track(
                        &id,
                        &pair.line1,
                        &pair.line2,
                        &pair.name,
                        now,
                        92,
                    ) {
                        orbits.push(track);
                    }
                }
            }
            Err(e) => log::warn!("Failed to get TLE for {cat_nr}: {e}"),
        }
    }

    Ok(SatelliteData { positions, orbits })
}

pub async fn get_pass_predictions_inner(db: &Database) -> Result<Vec<PassPrediction>, String> {
    let settings = db.get_settings();
    let user_lat = settings.user_lat.unwrap_or(37.3382);
    let user_lon = settings.user_lon.unwrap_or(-121.8863);

    let mut all_passes = Vec::new();

    for (cat_nr, url) in SATELLITES {
        match get_tle_cached(db, cat_nr, url).await {
            Ok(pairs) => {
                for pair in &pairs {
                    let id = format!("sat-{cat_nr}");
                    let passes = orbit::predict_passes(
                        &id,
                        &pair.name,
                        &pair.line1,
                        &pair.line2,
                        user_lat,
                        user_lon,
                        24,
                    );
                    all_passes.extend(passes);
                }
            }
            Err(e) => log::warn!("Failed to get TLE for {cat_nr}: {e}"),
        }
    }

    all_passes.sort_by_key(|pass| pass.start_time);
    Ok(all_passes)
}

#[tauri::command]
pub async fn get_satellite_positions(db: State<'_, Database>) -> Result<SatelliteData, String> {
    get_satellite_positions_inner(&db).await
}

#[tauri::command]
pub async fn get_pass_predictions(db: State<'_, Database>) -> Result<Vec<PassPrediction>, String> {
    get_pass_predictions_inner(&db).await
}
