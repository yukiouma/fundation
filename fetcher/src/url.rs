use std::time::{SystemTime, UNIX_EPOCH};

pub struct URLBuilder {
    base_url: String,
}

impl URLBuilder {
    pub fn new(base_url: &str) -> Self {
        URLBuilder {
            base_url: base_url.into(),
        }
    }
    pub fn url(&self, fond_code: &str, page: i64, pagesize: i64) -> anyhow::Result<String> {
        let end_date = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        Ok(format!(
            "{}?fundCode={}&pageIndex={}&pageSize={}&startDate&endDate&_={}",
            self.base_url, fond_code, page, pagesize, end_date
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn url_test() {
        let base = "https://api.fund.eastmoney.com/f10/lsjz";
        let builder = URLBuilder::new(base);
        println!("{}", builder.url("320007", 1, 20).unwrap());
    }
}
