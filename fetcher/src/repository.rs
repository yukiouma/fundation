use reqwest::{blocking::Client, Url};

use crate::dto::{Fund, ListFundsResponse, SaveNavsRequest};

const CONTENT_TYPE: &str = "Content-Type";
const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Clone)]
pub struct Repository {
    base_url: Url,
    client: Client,
}

impl Repository {
    pub fn new(base_url: &str) -> anyhow::Result<Self> {
        let base_url = Url::parse(base_url)?;
        let client = Client::new();
        Ok(Repository { base_url, client })
    }
    pub fn save_fund(&self, fund: &Fund) -> anyhow::Result<()> {
        let url = self.base_url.join("fund")?;
        let body = serde_json::to_vec(fund)?;
        self.client
            .post(url)
            .body(body)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()?;
        Ok(())
    }
    pub fn save_navs(&self, request: &SaveNavsRequest) -> anyhow::Result<()> {
        let url = self.base_url.join("nav")?;
        let body = serde_json::to_vec(request)?;
        self.client
            .post(url)
            .body(body)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()?;
        Ok(())
    }
    pub fn find_fund_by_code(&self, code: &str) -> anyhow::Result<Option<Fund>> {
        let url = self.base_url.join("fund")?;
        let response = self.client.get(url).query(&[("code", code)]).send()?;
        if response.status().ne(&reqwest::StatusCode::OK) {
            return Ok(None);
        }
        let bytes = response.bytes()?.to_vec();
        let fund: Fund = serde_json::from_slice(&bytes)?;
        Ok(Some(fund))
    }

    pub fn list_funds(&self) -> anyhow::Result<Vec<Fund>> {
        let url = self.base_url.join("fund/list")?;
        let response = self.client.get(url).send()?;
        let response: ListFundsResponse = serde_json::from_slice(&response.bytes()?.to_vec())?;
        Ok(response.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repostiory_test() {
        let fund = Fund {
            id: None,
            name: "诺安混合成长".into(),
            code: "320007".into(),
        };
        let base_url = "http://localhost:8080";
        let repo = Repository::new(base_url).unwrap();
        repo.save_fund(&fund).unwrap();

        let fund = repo.find_fund_by_code("320007").unwrap();
        assert_eq!(Some(1), fund.unwrap().id);
    }

    #[test]
    fn list_funds_test() {
        let base_url = "http://localhost:8080";
        let repo = Repository::new(base_url).unwrap();
        let funds = repo.list_funds().unwrap();
        println!("{:?}", funds);
        let expect = 2;
        assert_eq!(funds.len(), expect)
    }
}
