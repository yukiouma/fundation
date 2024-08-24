use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NavStatPanel {
    pub month: i32,
    pub code: String,
    pub name: String,
    pub max_nav: f64,
    pub min_nav: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindNavStatPanelRequest {
    pub code: Vec<String>,
    pub month: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindNavStatPanelReply {
    pub data: Vec<NavStatPanel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nav {
    pub fund_id: i64,
    pub date: i64,
    pub nav: f64,
    pub cnav: f64,
    pub growth: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLatestNavRequest {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLatestNavReply {
    pub data: Nav,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fund {
    pub id: i64,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListFundsReply {
    pub data: Vec<Fund>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddFundRequest {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveNavsRequest {
    pub data: Vec<Nav>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindFundByCodeRequest {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindFundByCodeReply {
    pub data: Fund,
}
