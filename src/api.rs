use actix_web::{error, Error, HttpResponse, Result, web};
use serde::{Deserialize, Serialize};

use crate::db;
use crate::models;

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub order: i32,
    pub url: String,
}

fn serialize(todo: &models::Todo) -> Todo {
    Todo {
        id: todo.id.clone(),
        title: todo.title.clone(),
        completed: todo.completed,
        order: todo.order.clone(),
        url: build_url(todo.id),
    }
}

fn build_url(id: i32) -> String {
    let base_url = std::env::var("BASE_URL").expect("BASE_URL");
    format!("{}/{}", base_url, id)
}

pub async fn index(
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    let todos = web::block(move || db::get_all_todos(&pool))
        .await
        .map(|todos| todos.iter().map(|todo| serialize(&todo)).collect::<Vec<Todo>>())
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(todos))
}

pub async fn create(
    pool: web::Data<db::PgPool>,
    form: web::Json<models::TodoCreate>,
) -> Result<HttpResponse, Error> {
    let inner_form = form.into_inner();
    let result = web::block(move || db::create_todo(inner_form.title,inner_form.order, &pool))
        .await
        .map(|todo| serialize(&todo))
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_all(
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || db::delete_all(&pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}

#[derive(Deserialize)]
pub struct UpdateParams {
    id: i32,
}

pub async fn get(
    pool: web::Data<db::PgPool>,
    params: web::Path<UpdateParams>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || db::get(params.id, &pool))
        .await
        .map(|todo| serialize(&todo))
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn update(
    pool: web::Data<db::PgPool>,
    params: web::Path<UpdateParams>,
    form: web::Json<models::TodoUpdate>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || db::update(params.id, form.into_inner(), &pool))
        .await
        .map(|todo| serialize(&todo))
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete(
    pool: web::Data<db::PgPool>,
    params: web::Path<UpdateParams>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || db::delete(params.id, &pool))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}