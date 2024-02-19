use anyhow::anyhow;
use chrono::{
    DateTime, Datelike, Days, LocalResult, NaiveDate, NaiveDateTime, TimeDelta, TimeZone, Timelike,
    Utc,
};
use chrono_tz::Tz;
use geocoding::{Forward, GeocodingError, Opencage, Point};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tzf_rs::DefaultFinder;

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::new();
}

pub fn get_tz_name(longitude: f64, latitude: f64) -> String {
    FINDER.get_tz_name(longitude, latitude).to_string()
}

pub fn naive_to_local_tz(
    local: NaiveDateTime,
    longitude: f64,
    latitude: f64,
) -> anyhow::Result<DateTime<Tz>> {
    let tz: Tz = get_tz_name(longitude, latitude)
        .parse::<Tz>()
        .map_err(|err| anyhow!("{}", err))?;

    match tz.from_local_datetime(&local) {
        LocalResult::None => Err("No valid timezone found".to_string()),
        LocalResult::Ambiguous(a, b) => {
            Err(format!("Two different datetimes are possible: {} {}", a, b))
        }
        LocalResult::Single(a) => Ok(a),
    }
    .map_err(|err| anyhow!("{}", err))
}

pub fn local_to_utc(
    local: NaiveDateTime,
    longitude: f64,
    latitude: f64,
) -> anyhow::Result<DateTime<Utc>> {
    let local_tz_aware = naive_to_local_tz(local, longitude, latitude)?;

    Ok(local_tz_aware.with_timezone(&Utc))
}

// TODO - need to throw this into a tokio::spawn thread or something
pub fn geocode(address: &String) -> anyhow::Result<Option<(f64, f64)>> {
    let token = std::env::var("OPENCAGE_API_TOKEN")?;
    let oc = Opencage::new(token);
    let res: Result<Vec<Point<f64>>, GeocodingError> = oc.forward(&address);

    if let Err(e) = &res {
        println!("Error while geocoding: {}", e);
        return Ok(None);
    }

    // Must be done after actually doing a geocoding call
    println!(
        "Geocoding calls remaining: {:?}",
        oc.remaining_calls().unwrap_or(0),
    );

    let first_result: Point<f64> = res.unwrap()[0];

    Ok(Some((first_result.x(), first_result.y())))
}

pub fn string_to_naive_datetime(s: String) -> anyhow::Result<NaiveDateTime> {
    NaiveDateTime::parse_from_str(&s, "%m/%d/%Y %I:%M %P").map_err(|err| anyhow!("{}", err))
}

pub fn string_to_naive_date(s: String) -> anyhow::Result<NaiveDate> {
    NaiveDate::parse_from_str(&s, "%m/%d/%Y").map_err(|err| anyhow!("{}", err))
}