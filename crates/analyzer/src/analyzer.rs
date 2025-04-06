use apis::repository::{FindNavStatPanelRequest, GetLatestNavRequest, RepositoryClient};
use chrono::DateTime;

pub struct Analyzer<'a> {
    client: &'a RepositoryClient,
}

#[derive(Debug)]
pub struct AnalyseResult {
    pub latest_price: String,
    pub latest_date: String,
    pub during_1_month: String,
    pub during_2_months: String,
    pub during_3_months: String,
    pub during_6_months: String,
    pub during_1_year: String,
    pub during_2_years: String,
    pub during_3_years: String,
}

impl<'a> Analyzer<'a> {
    pub fn new(client: &'a RepositoryClient) -> Self {
        Analyzer { client }
    }
    pub async fn analyse_fund(&self, fund_code: &str) -> anyhow::Result<AnalyseResult> {
        let latest_nav = self
            .client
            .get_latest_nav(&GetLatestNavRequest {
                code: fund_code.into(),
            })
            .await?;
        let panel = self
            .client
            .get_stat_panel(&FindNavStatPanelRequest {
                code: vec![fund_code.into()],
                month: vec![1, 2, 3, 6, 12, 24, 36],
            })
            .await?;
        let current_price = latest_nav.data.nav;
        // println!("\n{} - ({}):", fund.name, fund.code);
        let date = DateTime::from_timestamp_millis(latest_nav.data.date * 1000).unwrap();
        println!("current price({}): {}", date.to_string(), current_price);
        let mut during_1_month = String::new();
        let mut during_2_months = String::new();
        let mut during_3_months = String::new();
        let mut during_6_months = String::new();
        let mut during_1_year = String::new();
        let mut during_2_years = String::new();
        let mut during_3_years = String::new();
        panel.data.iter().for_each(|stat| {
            let water_mark = (current_price - stat.min_nav) / (stat.max_nav - stat.min_nav) * 100.0;
            let water_mark = format!("{:5.2}%", water_mark);
            // println!("[Month {:2}] {:5.2}%", stat.month, water_mark);
            match stat.month {
                1 => during_1_month = water_mark,
                2 => during_2_months = water_mark,
                3 => during_3_months = water_mark,
                6 => during_6_months = water_mark,
                12 => during_1_year = water_mark,
                24 => during_2_years = water_mark,
                36 => during_3_years = water_mark,
                _ => {}
            }
        });

        Ok(AnalyseResult {
            latest_price: format!("{}", current_price),
            latest_date: date.to_string().get(..10).unwrap().into(),
            during_1_month,
            during_2_months,
            during_3_months,
            during_6_months,
            during_1_year,
            during_2_years,
            during_3_years,
        })
    }
}

// pub async fn analyse() -> anyhow::Result<()> {
//     let client = RepositoryClient::new(&BASE_URL)?;
//     let fund_list = client.list_funds().await?.data;
//     for fund in fund_list {
//         let fund_code = fund.code.as_str();
//         let latest_nav = client
//             .get_latest_nav(&GetLatestNavRequest {
//                 code: fund_code.into(),
//             })
//             .await?;
//         let panel = client
//             .get_stat_panel(&FindNavStatPanelRequest {
//                 code: vec![fund_code.into()],
//                 month: vec![1, 2, 3, 6, 12, 24, 36],
//             })
//             .await?;
//         let current_price = latest_nav.data.nav;
//         println!("\n{} - ({}):", fund.name, fund.code);
//         let date = DateTime::from_timestamp_millis(latest_nav.data.date * 1000).unwrap();
//         println!("current price({}): {}", date.to_string(), current_price);
//         panel.data.iter().for_each(|stat| {
//             // let is_max = current_price.ge(&stat.max_nav);
//             // let is_min = current_price.le(&stat.min_nav);
//             let water_mark = (current_price - stat.min_nav) / (stat.max_nav - stat.min_nav) * 100.0;

//             println!("[Month {:2}] {:5.2}%", stat.month, water_mark);
//         });
//     }

//     Ok(())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn analyse_test() {
//         analyse().await.unwrap();
//     }
// }
