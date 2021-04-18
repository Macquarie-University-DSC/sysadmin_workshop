use chrono::naive::serde::ts_milliseconds;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use sqlx::Result;

#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    description: String,
    is_complete: bool,
    #[serde(with = "ts_milliseconds")]
    issue_date: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct NewTodo {
    description: String,
    is_complete: bool,
    #[serde(with = "ts_milliseconds")]
    issue_date: NaiveDateTime,
}

impl Todo {
    #[cfg(test)]
    pub fn new(id: i32, description: String, is_complete: bool, issue_date: NaiveDateTime) -> Todo {
        Todo {
            id,
            description,
            is_complete,
            issue_date,
        }
    }

    pub async fn create(todo: NewTodo, pool: &PgPool) -> Result<Todo> {
        let result = sqlx::query_as!(
            Todo,
            r#"
            INSERT INTO todo (description, is_complete, issue_date)
                VALUES ($1, $2, $3)
                RETURNING id, description, is_complete, issue_date
            "#,
            &todo.description,
            todo.is_complete,
            todo.issue_date
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn read_all(pool: &PgPool) -> Result<Vec<Todo>> {
        let result = sqlx::query_as!(Todo, "SELECT * FROM todo")
            .fetch_all(pool)
            .await?;

        Ok(result)
    }

    pub async fn read_by_id(id: i32, pool: &PgPool) -> Result<Todo> {
        let result = sqlx::query_as!(Todo, "SELECT * FROM todo WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    pub async fn update(id: i32, todo: NewTodo, pool: &PgPool) -> Result<Todo> {
        let result = sqlx::query_as!(
            Todo,
            r#"
            UPDATE todo 
                SET description = $2, is_complete = $3, issue_date = $4
                WHERE id = $1
                RETURNING id, description, is_complete, issue_date
            "#,
            id,
            &todo.description,
            todo.is_complete,
            todo.issue_date
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query!("DELETE FROM todo WHERE id = $1", id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

