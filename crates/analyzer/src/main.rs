use analyzer::Analyzer;
use apis::repository::RepositoryClient;

const BASE_URL: &'static str = "http://localhost:8080";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RepositoryClient::new(&BASE_URL)?;
    let analyzer = Analyzer::new(&client);
    let fund_list = client.list_funds().await?.data;
    for fund in fund_list {
        println!("{}({})", fund.name, fund.code);
        let result = analyzer.analyse_fund(&fund.code).await?;
        println!("{:?}", result);
    }
    Ok(())
}
