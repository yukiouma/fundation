use chrono::{TimeZone, Utc};
use dotenv::dotenv;
use fetcher::{Fetcher, Nav, Repository, SaveNavsRequest};
use std::{env, fs, ops::Div, path::Path, thread};
use tracing::{info, Level};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // 1. read config, then initialize the fetcher and repository
    dotenv().ok();
    let repo_base_url = env::var("REPO_BASE_URL")?;
    let fund_base_url = env::var("FUND_BASE_URL")?;
    let cookie_path = env::var("COOKIE_PATH")?;
    let cookie_path = Path::new(&cookie_path);
    let cookie = fs::read_to_string(cookie_path)?;
    let fetcher = Fetcher::new(&cookie, &fund_base_url);
    let repo = Repository::new(&repo_base_url)?;

    // 2. update all the fund existed in repository
    let funds = repo.list_funds()?;
    for fund in funds {
        let code = fund.code;
        if let Some(fund) = fetcher.fetch_fund_info_by_code(&code)? {
            repo.save_fund(&fund)?;
            // sleep for 1 second to avoid too many requests
            thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    // 3. update all the navs of each fond to latest day
    let funds = repo.list_funds()?;
    for fund in funds {
        let code = fund.code;
        info!("updating navs for fund: {}({})", fund.name, code);
        let navs = match repo.find_latest_nav(&code)? {
            Some(nav) => {
                let date = nav.date;
                let after = Utc.timestamp_opt(date, 0).unwrap();
                fetcher.fetch(&code, Some(after))?.data.value_list
            }
            None => fetcher.fetch(&code, None)?.data.value_list,
        };
        let navs = navs
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
    }
    Ok(())
}
