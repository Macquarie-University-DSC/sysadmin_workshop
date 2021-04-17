use actix_web::{delete, get, http::header, post, put, web, HttpResponse, Result};
use log::error;
use sqlx::error::Error;
use sqlx::postgres::PgPool;

use crate::model::{NewTodo, Todo};

#[post("/todo")]
pub async fn create_todo(
    new_todo: web::Json<NewTodo>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    let result = Todo::create(new_todo.into_inner(), db_pool.get_ref())
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/todos")]
pub async fn read_all_todos(db_pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let result = Todo::read_all(db_pool.get_ref()).await.map_err(|e| {
        error!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/todo/{id}")]
pub async fn read_todo_by_id(
    id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    let result = Todo::read_by_id(id.into_inner(), db_pool.get_ref())
        .await
        .map_err(|e| {
            error!("{}", e);
            match e {
                Error::RowNotFound => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError()
                    .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                    .body(e.to_string()),
            }
        })?;

    Ok(HttpResponse::Ok().json(result))
}

#[put("/todo/{id}")]
pub async fn update_todo(
    id: web::Path<i32>,
    new_todo: web::Json<NewTodo>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    let result = Todo::update(id.into_inner(), new_todo.into_inner(), db_pool.get_ref())
        .await
        .map_err(|e| {
            error!("{}", e);
            match e {
                Error::RowNotFound => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError()
                    .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                    .body(e.to_string()),
            }
        })?;

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/todo/{id}")]
pub async fn delete_todo(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> Result<HttpResponse> {
    Todo::delete(id.into_inner(), db_pool.get_ref())
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::NoContent().finish())
}

