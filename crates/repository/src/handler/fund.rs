use super::error::AppError;
use crate::{
    dto::{FindFundByCodeRequest, Fund, GeneralReply, ListFundsReply},
    DataRepo,
};
use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    Json,
};

pub async fn save_fund(
    State(repo): State<DataRepo>,
    Json(fund): Json<Fund>,
) -> Result<Json<GeneralReply>, AppError> {
    repo.save_fund(fund).await?;
    Ok(Json(GeneralReply {
        message: "success".into(),
    }))
}

pub async fn find_fund_by_code(
    request: Query<FindFundByCodeRequest>,
    State(repo): State<DataRepo>,
) -> anyhow::Result<Json<Fund>, AppError> {
    let fund = repo.find_fund_by_code(&request.code).await?;
    match fund {
        Some(fund) => Ok(Json(fund)),
        None => Err(anyhow!("Error: fund does not exist").into()),
    }
}

pub async fn list_funds(
    State(repo): State<DataRepo>,
) -> anyhow::Result<Json<ListFundsReply>, AppError> {
    let funds = repo.list_funds().await?;
    Ok(Json(ListFundsReply { data: funds }))
}
