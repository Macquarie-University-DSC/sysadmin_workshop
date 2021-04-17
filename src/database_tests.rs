use std::env;

use super::model::Todo;
use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;

#[async_std::test]
async fn read_all_todos() -> Result<()> {
    dotenv::dotenv().ok();

    let conn_str = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .after_connect(|conn| {
            Box::pin(async move {
                conn.execute("SET search_path = 'testing';").await?;
                Ok(())
            })
        })
        .connect(&conn_str)
        .await?;

    let result = Todo::read_all(&pool).await?;

    let fmt_str = "%Y-%m-%d %H:%M:%S";
    let expected = vec![
        Todo::new(
            1,
            String::from("test todo 1"),
            false,
            NaiveDateTime::parse_from_str("2021-04-17 20:00:00", &fmt_str)?,
        ),
        Todo::new(
            2,
            String::from("test todo 2"),
            false,
            NaiveDateTime::parse_from_str("2021-04-17 21:00:00", &fmt_str)?,
        ),
        Todo::new(
            3,
            String::from("test todo 3"),
            true,
            NaiveDateTime::parse_from_str("2021-04-17 10:00:00", &fmt_str)?,
        ),
        Todo::new(
            4,
            String::from("test todo 4"),
            true,
            NaiveDateTime::parse_from_str("2021-04-17 11:00:00", &fmt_str)?,
        ),
    ];

    assert_eq!(result, expected);

    Ok(())
}

#[async_std::test]
async fn read_todo_by_id() -> Result<()> {
    dotenv::dotenv().ok();

    let conn_str = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .after_connect(|conn| {
            Box::pin(async move {
                conn.execute("SET search_path = 'testing';").await?;
                Ok(())
            })
        })
        .connect(&conn_str)
        .await?;

    let result = Todo::read_by_id(3, &pool).await?;

    let expected = Todo::new(
        3,
        String::from("test todo 3"),
        true,
        NaiveDateTime::parse_from_str("2021-04-17 10:00:00", "%Y-%m-%d %H:%M:%S")?,
    );

    assert_eq!(result, expected);

    Ok(())
}

