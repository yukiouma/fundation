use std::{
    env, fs,
    ops::{Div, Rem},
    path::Path,
};

use dotenv::dotenv;
use fetcher::{Fetcher, Nav, Repository, SaveNavsRequest, URLBuilder};

const PAGESIZE: i64 = 1000;

fn main() -> anyhow::Result<()> {
    // 1. read config, then initialize the fetcher and repository
    dotenv().ok();
    let repo_base_url = env::var("REPO_BASE_URL")?;
    let fund_base_url = env::var("FUND_BASE_URL")?;
    let cookie_path = env::var("COOKIE_PATH")?;
    let cookie_path = Path::new(&cookie_path);
    let cookie = fs::read_to_string(cookie_path)?;
    let fetcher = Fetcher::new(&cookie);
    let repo = Repository::new(&repo_base_url)?;

    // 2. update all the fund existed in repository
    let funds = repo.list_funds()?;
    for fund in funds {
        let code = fund.code;
        if let Some(fund) = fetcher.fetch_fund_info_by_code(&code)? {
            repo.save_fund(&fund)?;
        }
    }

    // 3. update all the navs of each fond to latest day
    let funds = repo.list_funds()?;
    let url_builder = URLBuilder::new(&fund_base_url);
    for fund in funds {
        let code = fund.code;
        println!("updating navs for fund: {}", code);
        let url = url_builder.url(&code, 1, 20)?;
        let navs = fetcher.fetch(&url)?;
        let total_count = navs.total_count;
        let mut pages =
            total_count.div(PAGESIZE) + if total_count.rem(PAGESIZE) > 0 { 1 } else { 0 };
        println!("total count: {}, total pages: {}", total_count, pages);
        while pages > 0 {
            let url = url_builder.url(&code, pages, PAGESIZE)?;
            let navs = fetcher
                .fetch(&url)?
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
            repo.save_navs(&SaveNavsRequest { data: navs })?;
            pages -= 1;
        }
    }
    Ok(())
}
