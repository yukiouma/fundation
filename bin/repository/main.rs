use axum::{
    extract::{MatchedPath, Request},
    routing::{get, post},
    Router,
};
use repository::{
    handler::{fund, fund_nav},
    DataRepo,
};
use sqlx::MySqlPool;
use std::env;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_url = env::var(&"DATABASE_URL")?;
    let pool = MySqlPool::connect(&database_url).await.unwrap();
    let repo = DataRepo::new(pool);
    let app = Router::new()
        .route("/fund", post(fund::save_fund).get(fund::find_fund_by_code))
        .route(
            "/nav",
            post(fund_nav::save_nav).get(fund_nav::find_latest_nav_date),
        )
        .route("/nav/latest", get(fund_nav::find_latest_nav_by_code))
        .route("/stat", post(fund_nav::find_nav_stat_panel))
        .route("/fund/list", get(fund::list_funds))
        .with_state(repo)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);
                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty
                )
            }),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    info!("Fundation Repository Server Start");
    axum::serve(listener, app).await?;
    Ok(())
}
