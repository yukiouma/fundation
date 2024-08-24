use reqwest::{header::CONTENT_TYPE, Client};
use url::Url;

use super::{
    dto::{FindNavStatPanelReply, FindNavStatPanelRequest, GetLatestNavReply, GetLatestNavRequest},
    AddFundRequest, FindFundByCodeReply, FindFundByCodeRequest, Fund, ListFundsReply,
    SaveNavsRequest,
};

const APPLICATION_JSON: &str = "application/json";

pub struct RepositoryClient {
    base_url: Url,
    client: Client,
}

impl RepositoryClient {
    pub fn new(base_url: &str) -> anyhow::Result<RepositoryClient> {
        let base_url = Url::parse(base_url)?;
        let client = reqwest::Client::new();
        Ok(RepositoryClient { base_url, client })
    }
    pub async fn get_stat_panel(
        &self,
        request: &FindNavStatPanelRequest,
    ) -> anyhow::Result<FindNavStatPanelReply> {
        let url = self.base_url.join("stat")?;
        let bytes = self
            .client
            .post(url)
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_vec(&request)?)
            .send()
            .await?
            .bytes()
            .await?;
        let reply = serde_json::from_slice::<FindNavStatPanelReply>(&bytes.to_vec())?;
        Ok(reply)
    }

    pub async fn get_latest_nav(
        &self,
        request: &GetLatestNavRequest,
    ) -> anyhow::Result<GetLatestNavReply> {
        let mut url = self.base_url.join("nav/")?.join("latest")?;
        url.set_query(Some(&format!("code={}", request.code)));
        let bytes = self.client.get(url).send().await?.bytes().await?;
        let reply = serde_json::from_slice::<GetLatestNavReply>(&bytes.to_vec())?;
        Ok(reply)
    }

    pub async fn list_funds(&self) -> anyhow::Result<ListFundsReply> {
        let url = self.base_url.join("fund/")?.join("list")?;
        let bytes = self.client.get(url).send().await?.bytes().await?;
        let reply = serde_json::from_slice::<ListFundsReply>(&bytes.to_vec())?;
        Ok(reply)
    }

    pub async fn add_fund(&self, request: &AddFundRequest) -> anyhow::Result<()> {
        let url = self.base_url.join("fund")?;
        self.client
            .post(url)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .body(serde_json::to_vec(request)?)
            .send()
            .await?;
        Ok(())
    }

    pub async fn save_navs(&self, request: &SaveNavsRequest) -> anyhow::Result<()> {
        let url = self.base_url.join("nav")?;
        let body = serde_json::to_vec(request)?;
        self.client
            .post(url)
            .body(body)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()
            .await?;
        Ok(())
    }

    pub async fn find_fund_by_code(
        &self,
        request: &FindFundByCodeRequest,
    ) -> anyhow::Result<Option<FindFundByCodeReply>> {
        let url = self.base_url.join("fund")?;
        let response = self
            .client
            .get(url)
            .query(&[("code", &request.code)])
            .send()
            .await?;
        if response.status().ne(&reqwest::StatusCode::OK) {
            return Ok(None);
        }
        let bytes = response.bytes().await?.to_vec();
        let fund: Fund = serde_json::from_slice(&bytes)?;
        Ok(Some(FindFundByCodeReply { data: fund }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn fetch() {
        let base_url = "http://localhost:8080";
        let request = FindNavStatPanelRequest {
            month: vec![1, 2, 3, 6, 12, 24, 36],
            code: vec!["320007".into()],
        };
        let client = RepositoryClient::new(base_url).unwrap();
        let panel = client.get_stat_panel(&request).await;
        assert!(panel.is_ok());

        let request = GetLatestNavRequest {
            code: "320007".into(),
        };
        let latest_nav = client.get_latest_nav(&request).await;
        assert!(latest_nav.is_ok());

        let fund_list = client.list_funds().await;
        assert!(fund_list.is_ok());

        let reply = client
            .add_fund(&AddFundRequest {
                name: "".into(),
                code: "159513".into(),
            })
            .await;
        assert!(reply.is_ok());
    }
}
