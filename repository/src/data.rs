use chrono::{DateTime, Utc};
use fund::{find_fund_by_code, list_funds, save_fund};
use fund_nav::{
    create_stat_panel, find_latest_nav, find_latest_nav_date, find_navs_stat,
    nav_stat_panel_existed, save_nav,
};
use sqlx::{MySql, Pool};

use crate::dto::{Fund, Nav, NavStatPanel};

mod fund;
mod fund_nav;

#[derive(Clone)]
pub struct DataRepo {
    pool: Pool<MySql>,
}

impl DataRepo {
    pub fn new(pool: Pool<MySql>) -> Self {
        DataRepo { pool }
    }
    pub async fn save_fund(&self, fund: Fund) -> anyhow::Result<()> {
        save_fund(&self.pool, fund).await?;
        Ok(())
    }
    pub async fn find_fund_by_code(&self, code: &str) -> anyhow::Result<Option<Fund>> {
        find_fund_by_code(&self.pool, code).await
    }
    pub async fn save_navs(&self, navs: &[Nav]) -> anyhow::Result<()> {
        save_nav(&self.pool, navs).await?;
        Ok(())
    }
    pub async fn find_latest_nav_date(&self, code: &str) -> anyhow::Result<Option<DateTime<Utc>>> {
        let fund = find_fund_by_code(&self.pool, code).await?;
        if let None = fund {
            return Ok(None);
        }
        let fund_id = fund.unwrap().id.unwrap();
        Ok(find_latest_nav_date(&self.pool, fund_id).await?)
    }

    pub async fn find_latest_nav(&self, code: &str) -> anyhow::Result<Option<Nav>> {
        let fund = find_fund_by_code(&self.pool, code).await?;
        match fund {
            Some(fund) => {
                let nav = find_latest_nav(&self.pool, fund.id.unwrap()).await?;
                Ok(nav)
            }
            None => Ok(None),
        }
    }

    pub async fn find_nav_stat_panel(
        &self,
        code: &[String],
        month: usize,
    ) -> anyhow::Result<Vec<NavStatPanel>> {
        let exist = nav_stat_panel_existed(&self.pool, month).await?;
        if !exist {
            create_stat_panel(&self.pool, month).await?;
        }
        let nav_stat = find_navs_stat(&self.pool, code, month).await?;
        Ok(nav_stat)
    }
    pub async fn list_funds(&self) -> anyhow::Result<Vec<Fund>> {
        let funds = list_funds(&self.pool).await?;
        Ok(funds)
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Days};
    use sqlx::MySqlPool;

    use super::*;

    #[tokio::test]
    async fn nav_panel_test() {
        let database_url = "mysql://root:000000@localhost:3306/fundation?parseTime=True";
        let pool = MySqlPool::connect(database_url).await.unwrap();
        let repo = DataRepo::new(pool);
        let code = vec!["161725".to_string()];
        let result = repo.find_nav_stat_panel(&code, 1).await.unwrap();
        println!("{:?}", result);
        let result = repo.find_nav_stat_panel(&code, 3).await.unwrap();
        println!("{:?}", result);
        let result = repo.find_nav_stat_panel(&code, 6).await.unwrap();
        println!("{:?}", result);
        let result = repo.find_nav_stat_panel(&code, 12).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn repo_test() {
        let database_url = "mysql://root:000000@localhost:3306/fundation?parseTime=True";
        let pool = MySqlPool::connect(database_url).await.unwrap();
        let repo = DataRepo::new(pool);
        let code = "320007";
        let fund = Fund {
            name: "诺安混合成长".into(),
            code: code.into(),
            ..Default::default()
        };
        repo.save_fund(fund).await.unwrap();
        let fund = repo.find_fund_by_code(code).await.unwrap().unwrap();
        let fund_id = fund.id.unwrap();
        let timestamp = DateTime::from_timestamp_millis(1719843534000).unwrap();
        let navs = vec![
            Nav {
                fund_id,
                growth: 1.0,
                nav: 1.0,
                cnav: 1.0,
                date: timestamp.checked_sub_days(Days::new(0)).unwrap(),
                ..Default::default()
            },
            Nav {
                fund_id,
                growth: 2.0,
                nav: 2.0,
                cnav: 2.0,
                date: timestamp.checked_sub_days(Days::new(4)).unwrap(),
                ..Default::default()
            },
        ];
        repo.save_navs(&navs).await.unwrap();
        let latest = repo.find_latest_nav_date(code).await.unwrap();
        assert_eq!(latest.unwrap(), timestamp);
    }
}
