use sqlx::{MySql, Pool};

use crate::dto::Fund;

pub async fn save_fund(pool: &Pool<MySql>, fund: Fund) -> anyhow::Result<()> {
    match find_fund_by_code(pool, &fund.code).await? {
        Some(data) => {
            let id = data.id;
            update_fund(
                pool,
                Fund {
                    id,
                    name: fund.name,
                    code: fund.code,
                },
            )
            .await?;
        }
        None => {
            create_fund(pool, fund).await?;
        }
    }
    Ok(())
}

pub async fn find_fund_by_code(pool: &Pool<MySql>, code: &str) -> anyhow::Result<Option<Fund>> {
    match sqlx::query!(
        "SELECT `id`, `name`, `code` FROM fund WHERE `code` = ?;",
        code
    )
    .fetch_optional(pool)
    .await?
    {
        Some(data) => Ok(Some(Fund {
            id: Some(data.id),
            name: data.name,
            code: data.code,
        })),
        None => Ok(None),
    }
}

pub async fn list_funds(pool: &Pool<MySql>) -> anyhow::Result<Vec<Fund>> {
    let funds = sqlx::query_as::<_, Fund>("SELECT `id`, `code`, `name` FROM `fund`")
        .fetch_all(pool)
        .await?;
    Ok(funds)
}

async fn create_fund(pool: &Pool<MySql>, fund: Fund) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO `fund` (`name`, `code`) VALUE (?, ?)",
        fund.name,
        fund.code
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn update_fund(pool: &Pool<MySql>, fund: Fund) -> anyhow::Result<()> {
    sqlx::query!(
        "UPDATE `fund` SET `name` = ? WHERE `id` = ?",
        fund.name,
        fund.id
    )
    .execute(pool)
    .await?;
    Ok(())
}
