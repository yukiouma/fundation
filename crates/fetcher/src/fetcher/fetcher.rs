use super::{
    dto::{Data, Fund, Response},
    header::header_map,
    URLBuilder,
};
use chrono::{DateTime, Datelike, Utc};
use regex::Regex;
use reqwest::{blocking::Client, header::HeaderMap};
use std::{thread, time::Duration};

const PAGESIZE: i64 = 20;

pub struct Fetcher {
    client: Client,
    headers: HeaderMap,
    url_builder: URLBuilder,
}

impl Fetcher {
    pub fn new(cookie: &str, base_url: &str) -> Fetcher {
        let client = reqwest::blocking::Client::new();
        let headers = header_map(cookie);
        let url_builder = URLBuilder::new(base_url);
        Fetcher {
            client,
            headers,
            url_builder,
        }
    }

    pub fn fetch(&self, fund_code: &str, after: Option<DateTime<Utc>>) -> anyhow::Result<Response> {
        Ok(match after {
            Some(after) => {
                let after = format!("{}-{:02}-{:02}", after.year(), after.month(), after.day());
                let url = self.url_builder.url(fund_code, 1, PAGESIZE, Some(&after))?;
                let body = self
                    .client
                    .get(url)
                    .headers(self.headers.clone())
                    .send()?
                    .bytes()?;
                let data = serde_json::from_slice::<Response>(&body)?;
                data
            }
            None => self.fetch_all(fund_code)?,
        })
    }

    fn fetch_one_page(&self, fund_code: &str, page: i64) -> anyhow::Result<Response> {
        let url = self.url_builder.url(fund_code, page, PAGESIZE, None)?;
        let body = self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()?
            .bytes()?;
        let data = serde_json::from_slice::<Response>(&body)?;
        Ok(data)
    }

    fn fetch_all(&self, fund_code: &str) -> anyhow::Result<Response> {
        let mut page = 1;
        let mut data = vec![];
        let reply = self.fetch_one_page(fund_code, page)?;
        let mut values = reply.data.value_list;
        let mut total_count = reply.total_count;
        data.append(&mut values);
        if total_count.gt(&PAGESIZE) {
            while total_count > 0 {
                page += 1;
                let reply = self.fetch_one_page(fund_code, page)?;
                let mut values = reply.data.value_list;
                data.append(&mut values);
                total_count -= PAGESIZE;
                // sleep for 1 second
                thread::sleep(Duration::from_secs(1));
            }
        }
        Ok(Response {
            data: Data { value_list: data },
            total_count: reply.total_count,
        })
    }

    pub fn fetch_fund_info_by_code(&self, code: &str) -> anyhow::Result<Option<Fund>> {
        let url = self.url_builder.metadata_url(code)?;
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
