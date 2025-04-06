use super::error::AppError;
use crate::{
    dto::{
        FindLatestNavDateReply, FindLatestNavReply, FindLatestNavRequest, FindNavStatPanelReply,
        FindNavStatPanelRequest, GeneralReply, SaveNavRequest,
    },
    DataRepo,
};
use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    Json,
};

pub async fn save_nav(
    State(repo): State<DataRepo>,
    Json(request): Json<SaveNavRequest>,
) -> Result<Json<GeneralReply>, AppError> {
    repo.save_navs(&request.data).await?;
    Ok(Json(GeneralReply {
        message: "success".into(),
    }))
}

pub async fn find_latest_nav_date(
    State(repo): State<DataRepo>,
    Query(request): Query<FindLatestNavRequest>,
) -> Result<Json<FindLatestNavDateReply>, AppError> {
    match repo.find_latest_nav_date(&request.code).await? {
        Some(timestamp) => Ok(Json(FindLatestNavDateReply {
            date: Some(format!("{}", timestamp.format("%Y-%m-%d"))),
        })),
        None => Ok(Json(FindLatestNavDateReply { date: None })),
    }
}

pub async fn find_latest_nav_by_code(
    State(repo): State<DataRepo>,
    Query(request): Query<FindLatestNavRequest>,
) -> anyhow::Result<Json<FindLatestNavReply>, AppError> {
    let code = request.code;
    let nav = repo.find_latest_nav(&code).await?;
    match nav {
        Some(nav) => Ok(Json(FindLatestNavReply { data: nav })),
        None => {
            let e = AppError(anyhow!("fund does not existed"));
            Err(e.into())
        }
    }
}

pub async fn find_nav_stat_panel(
    State(repo): State<DataRepo>,
    Json(query): Json<FindNavStatPanelRequest>,
) -> anyhow::Result<Json<FindNavStatPanelReply>, AppError> {
    let FindNavStatPanelRequest { code, month } = query;
    let mut panels = vec![];
    for m in month {
        let panel = repo.find_nav_stat_panel(&code, m).await?;
        panel.into_iter().for_each(|p| panels.push(p));
    }
    Ok(Json(FindNavStatPanelReply { data: panels }))
}
