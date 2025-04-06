mod fetcher;
mod repository;

pub use fetcher::{Fetcher, Nav, SaveNavsRequest, URLBuilder};
pub use repository::Repository;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ops::{Div, Rem},
        thread,
        time::Duration,
    };

    #[test]
    fn it_works() {
        let cookie = "AUTH_FUND.EASTMONEY.COM_GSJZ=AUTH*TTJJ*TOKEN; Eastmoney_Fund=012733_161628_004854_162412_007301_161725_004851_320007_012768_562510_008282_002963; qgqp_b_id=04eb6936096b1673a379b5dec74b6a6b; websitepoptg_api_time=1718804955155; st_si=23226155607772; st_asi=delete; EMFUND1=null; EMFUND2=null; EMFUND3=null; EMFUND4=null; EMFUND5=null; EMFUND6=null; EMFUND7=null; EMFUND8=null; EMFUND0=null; EMFUND9=06-19 21:49:26@#$%u8BFA%u5B89%u6210%u957F%u6DF7%u5408@%23%24320007; st_pvi=97983226879462; st_sp=2022-09-21%2022%3A12%3A05; st_inirUrl=https%3A%2F%2Fwww.google.com%2F; st_sn=6; st_psi=20240619215243472-112200305283-0057308270";

        let base_url = "https://api.fund.eastmoney.com/f10/lsjz";
        let fetcher = Fetcher::new(&cookie, base_url);
        let builder = URLBuilder::new(base_url);
        let fond_codes = vec!["320007", "161725"];

        let repo = Repository::new("http://localhost:8080").unwrap();
        let pagesize = 1000;
        for code in fond_codes {
            let url = builder.url(code, 1, 20, None).unwrap();
            let fund = repo.find_fund_by_code(code).unwrap();
            if let None = fund {
                let fund = fetcher.fetch_fund_info_by_code(code).unwrap();
                if let Some(fund) = fund {
                    repo.save_fund(&fund).unwrap();
                }
            }
            let fund = repo.find_fund_by_code(code).unwrap().unwrap();
            let resp = fetcher.fetch(&url, None).unwrap();
            let total_count = resp.total_count;
            let mut page = total_count.div(pagesize);
            if total_count.rem(pagesize) > 0 {
                page += 1;
            }
            loop {
                if page == 0 {
                    break;
                }
                let url = builder.url(code, page, pagesize, None).unwrap();
                let resp = fetcher.fetch(&url, None).unwrap();
                let navs = resp
                    .data
                    .value_list
                    .iter()
                    .map(|nav| Nav {
                        fund_id: fund.id.unwrap(),
                        nav: nav.nav,
                        date: nav.date.div(1000),
                        cnav: nav.cnav,
                        growth: nav.growth,
                    })
                    .collect::<Vec<Nav>>();
                repo.save_navs(&SaveNavsRequest { data: navs }).unwrap();
                page -= 1;
                thread::sleep(Duration::from_secs(3));
            }
        }
    }
}
