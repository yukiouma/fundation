mod dto;
mod fetcher;
mod header;
mod serialize;
mod url;

pub use dto::{Fund, ListFundsResponse, Nav, SaveNavsRequest};
pub use fetcher::Fetcher;
pub use url::URLBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fetcher_all() {
        let f = Fetcher::new(&cookie(), "https://api.fund.eastmoney.com/f10/lsjz");
        let result = f.fetch("159632", None).unwrap();
        assert!(result.data.value_list.len() > 0);
        println!("{:?}", result);
    }

    #[test]
    fn test_fetcher_after() {
        let f = Fetcher::new(&cookie(), "https://api.fund.eastmoney.com/f10/lsjz");
        let after = chrono::Utc::now() - chrono::Duration::days(30);
        let result = f.fetch("159632", Some(after)).unwrap();
        assert!(result.data.value_list.len() > 0);
        println!("{:?}", result);
    }

    fn cookie() -> String {
        "AUTH_FUND.EASTMONEY.COM_GSJZ=AUTH*TTJJ*TOKEN; Eastmoney_Fund=012733_161628_004854_162412_007301_161725_004851_320007_012768_562510_008282_002963; qgqp_b_id=04eb6936096b1673a379b5dec74b6a6b; websitepoptg_api_time=1718804955155; st_si=23226155607772; st_asi=delete; EMFUND1=null; EMFUND2=null; EMFUND3=null; EMFUND4=null; EMFUND5=null; EMFUND6=null; EMFUND7=null; EMFUND8=null; EMFUND0=null; EMFUND9=06-19 21:49:26@#$%u8BFA%u5B89%u6210%u957F%u6DF7%u5408@%23%24320007; st_pvi=97983226879462; st_sp=2022-09-21%2022%3A12%3A05; st_inirUrl=https%3A%2F%2Fwww.google.com%2F; st_sn=6; st_psi=20240619215243472-112200305283-0057308270".to_string()
    }
}
