use chrono::{Local, TimeZone};
use serde::{de, Deserialize, Deserializer};

pub fn f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.parse::<f64>() {
        Ok(n) => Ok(n),
        Err(_) => Ok(0f64),
    }
    // s.parse::<f64>().map_err(de::Error::custom)
}

pub fn timestamp_from_str<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let ymd = s.split('-').collect::<Vec<&str>>();

    let year = ymd[0].parse::<i32>().map_err(de::Error::custom)?;
    let month = ymd[1].parse::<u32>().map_err(de::Error::custom)?;
    let day = ymd[2].parse::<u32>().map_err(de::Error::custom)?;

    match Local.with_ymd_and_hms(year, month, day, 0, 0, 0) {
        chrono::offset::LocalResult::Single(datetime) => Ok(datetime.timestamp_millis()),
        chrono::offset::LocalResult::Ambiguous(datetime, _) => Ok(datetime.timestamp_millis()),
        chrono::offset::LocalResult::None => Err(de::Error::custom("failed to parse date time")),
    }
}
