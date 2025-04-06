use super::serialize::{f64_from_str, timestamp_from_str};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Value {
    #[serde(deserialize_with = "timestamp_from_str", rename(deserialize = "FSRQ"))]
    // #[serde(rename())]
    pub date: i64,
    #[serde(deserialize_with = "f64_from_str", rename(deserialize = "DWJZ"))]
    pub nav: f64,
    #[serde(deserialize_with = "f64_from_str", rename(deserialize = "LJJZ"))]
    pub cnav: f64,
    #[serde(deserialize_with = "f64_from_str", rename(deserialize = "JZZZL"))]
    pub growth: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    #[serde(rename(deserialize = "LSJZList"))]
    pub value_list: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    #[serde(rename(deserialize = "Data"))]
    pub data: Data,
    #[serde(rename(deserialize = "TotalCount"))]
    pub total_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fund {
    pub id: Option<i64>,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Nav {
    pub fund_id: i64,
    pub date: i64,
    pub nav: f64,
    pub cnav: f64,
    pub growth: f64,
}

#[derive(Debug, Serialize)]
pub struct SaveNavsRequest {
    pub data: Vec<Nav>,
}

#[derive(Debug, Deserialize)]
pub struct ListFundsResponse {
    pub data: Vec<Fund>,
}
