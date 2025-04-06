pub struct URLBuilder {
    base_url: String,
}

impl URLBuilder {
    pub fn new(base_url: &str) -> Self {
        URLBuilder {
            base_url: base_url.into(),
        }
    }
    pub fn url(
        &self,
        fond_code: &str,
        page: i64,
        pagesize: i64,
        after: Option<&str>,
    ) -> anyhow::Result<String> {
        let start_date = after.unwrap_or_default();
        Ok(format!(
            "{}?fundCode={}&pageIndex={}&pageSize={}&startDate={}",
            self.base_url, fond_code, page, pagesize, start_date
        ))
    }

    pub fn metadata_url(&self, fund_code: &str) -> anyhow::Result<String> {
        let url = format!("https://fund.eastmoney.com/pingzhongdata/{}.js", fund_code);
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn url_test() {
        let base = "https://api.fund.eastmoney.com/f10/lsjz";
        let builder = URLBuilder::new(base);
        println!("{}", builder.url("320007", 1, 20, None).unwrap());
    }
}
