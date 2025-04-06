use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use sqlx::prelude::FromRow;

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Fund {
    pub id: Option<i64>,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Nav {
    pub id: Option<i64>,
    pub fund_id: i64,
    #[serde(
        deserialize_with = "timestamp_deserialize",
        serialize_with = "timestamp_serialize"
    )]
    pub date: DateTime<Utc>,
    pub nav: f64,
    pub cnav: f64,
    pub growth: f64,
}

#[derive(Debug, Serialize)]
pub struct GeneralReply {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct SaveNavRequest {
    pub data: Vec<Nav>,
}

#[derive(Debug, Deserialize)]
pub struct FindLatestNavRequest {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct FindFundByCodeRequest {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct FindNavStatPanelRequest {
    pub code: Vec<String>,
    pub month: Vec<usize>,
}

#[derive(Debug, Serialize)]
pub struct FindNavStatPanelReply {
    pub data: Vec<NavStatPanel>,
}

#[derive(Debug, Serialize)]
pub struct FindLatestNavDateReply {
    pub date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FindLatestNavReply {
    pub data: Nav,
}

#[derive(Debug, Serialize)]
pub struct ListFundsReply {
    pub data: Vec<Fund>,
}

pub fn timestamp_deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    match DateTime::<Utc>::from_timestamp(i64::deserialize(deserializer)?, 0) {
        Some(timestamp) => Ok(timestamp),
        None => Err(de::Error::custom("failed to parse date time")),
    }
}

pub fn timestamp_serialize<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let timestamp = value.timestamp();
    serializer.serialize_i64(timestamp)
}

#[derive(Debug, Serialize, FromRow)]
pub struct NavStatPanel {
    pub month: i32,
    pub code: String,
    pub name: String,
    pub max_nav: f64,
    pub min_nav: f64,
}
