use chrono::{DateTime, Utc};
use sqlx::{query, query_as, MySql, Pool};

use crate::dto::{Nav, NavStatPanel};

pub async fn save_nav(pool: &Pool<MySql>, nav_list: &[Nav]) -> anyhow::Result<()> {
    if nav_list.is_empty() {
        return Ok(());
    }
    if let Some(latest) = find_latest_nav_date(pool, nav_list.first().unwrap().fund_id).await? {
        let new_nav_list = nav_list
            .iter()
            .filter(|nav| nav.date.gt(&latest))
            .collect::<Vec<&Nav>>();
        create_navs(pool, &new_nav_list).await?;
    } else {
        let new_nav_list = nav_list.iter().collect::<Vec<&Nav>>();
        create_navs(pool, &new_nav_list).await?;
    }
    Ok(())
}

pub async fn find_latest_nav_date(
    pool: &Pool<MySql>,
    fund_id: i64,
) -> anyhow::Result<Option<DateTime<Utc>>> {
    match sqlx::query!(
        "SELECT `date` FROM `fund_nav` WHERE fund_id = ? ORDER BY `date` DESC LIMIT 1",
        fund_id
    )
    .fetch_optional(pool)
    .await?
    {
        Some(record) => Ok(Some(record.date)),
        None => Ok(None),
    }
}

pub async fn find_latest_nav(pool: &Pool<MySql>, fund_id: i64) -> anyhow::Result<Option<Nav>> {
    let nav = sqlx::query_as::<_, Nav>(
        "SELECT `id`, `fund_id`, `date`, `nav`, `cnav`, `growth` FROM `fund_nav` WHERE fund_id = ? ORDER BY `date` DESC LIMIT 1",
    )
    .bind(fund_id)
    .fetch_optional(pool)
    .await?;

    Ok(nav)
}

async fn create_navs(pool: &Pool<MySql>, nav_list: &[&Nav]) -> anyhow::Result<()> {
    if nav_list.is_empty() {
        return Ok(());
    }
    let mut insert_syntax =
        String::from("INSERT INTO `fund_nav` (`fund_id`, `date`, `nav`, `cnav`, `growth`) VALUES");
    let insert_values = nav_list
        .iter()
        .map(|nav| {
            format!(
                " ('{}', '{}', '{}', '{}', '{}')",
                nav.fund_id,
                nav.date.format("%Y-%m-%d %H:%M:%S"),
                nav.nav,
                nav.cnav,
                nav.growth
            )
        })
        .collect::<Vec<String>>()
        .join(",");
    insert_syntax.push_str(&insert_values);
    sqlx::query(&insert_syntax).execute(pool).await?;
    Ok(())
}

pub async fn find_navs_stat(
    pool: &Pool<MySql>,
    code: &[String],
    month: usize,
) -> anyhow::Result<Vec<NavStatPanel>> {
    let view_name = view_name(month);
    let syntax =
        format!(
        "SELECT `code`, `name`, `max_nav`, `min_nav`, {} AS `month` FROM `{}` WHERE `code` in {}",
        month, view_name,format!("('{}')", code.join("', '"))
    );
    let panel = query_as::<_, NavStatPanel>(&syntax).fetch_all(pool).await?;
    Ok(panel)
}

pub async fn nav_stat_panel_existed(pool: &Pool<MySql>, month: usize) -> anyhow::Result<bool> {
    let view_name = view_name(month);
    let syntax = format!(
        "SELECT `code`, `name`, `max_nav`, `min_nav` FROM {}",
        view_name
    );
    match query(&syntax).fetch_one(pool).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub async fn create_stat_panel(pool: &Pool<MySql>, month: usize) -> anyhow::Result<()> {
    let view_name = view_name(month);
    let syntax = format!(
        r#"
 CREATE VIEW `{}` AS
SELECT `fund`.`id`,
    `fund`.`code`,
    `fund`.`name`,
    `nav`.`max_nav`,
    `nav`.`min_nav`
FROM `fund` AS fund
    LEFT JOIN (
        SELECT `fund_id`,
            MAX(`nav`) AS max_nav,
            MIN(`nav`) AS min_nav
        FROM `fund_nav`
        WHERE `date` >= DATE_SUB(CURDATE(), INTERVAL {} MONTH)
        GROUP BY `fund_id`
    ) as nav on `fund`.`id` = `nav`.`fund_id`;   
    "#,
        view_name, month
    );
    query(&syntax).execute(pool).await?;
    Ok(())
}

fn view_name(month: usize) -> String {
    format!("latest_{}_month_nav", month)
}
