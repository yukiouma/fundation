use crate::{dto::Fund, header::header_map};

use super::dto::Response;

use regex::Regex;
use reqwest::{blocking::Client, header::HeaderMap};

pub struct Fetcher {
    client: Client,
    headers: HeaderMap,
}

impl Fetcher {
    pub fn new(cookie: &str) -> Fetcher {
        let client = reqwest::blocking::Client::new();
        let headers = header_map(cookie);
        Fetcher { client, headers }
    }

    pub fn fetch(&self, url: &str) -> anyhow::Result<Response> {
        let body = self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()?
            .bytes()?;
        let data = serde_json::from_slice::<Response>(&body)?;
        Ok(data)
    }

    pub fn fetch_fund_info_by_code(&self, code: &str) -> anyhow::Result<Option<Fund>> {
        let url = format!("https://fund.eastmoney.com/pingzhongdata/{}.js", code);
        // extract the fund name from js script
        let js = self
            .client
            .get(url)
            // .headers(self.headers.clone())
            .send()?
            .text()?;

        let re = Regex::new(r#"var fS_name = "(.+?)""#)?;
        match re.captures(&js) {
            Some(capture) => match capture.get(1) {
                Some(name) => Ok(Some(Fund {
                    id: None,
                    name: name.as_str().into(),
                    code: code.into(),
                })),
                None => Ok(None),
            },
            None => Ok(None),
        }
    }
}
