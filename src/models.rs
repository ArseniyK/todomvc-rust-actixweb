use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    todos,
    todos::dsl::todos as all_todos,
};

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub order: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="todos"]
pub struct TodoCreate {
    pub title: String,
    pub order: Option<i32>,
}

#[derive(Deserialize, Clone, AsChangeset)]
#[table_name="todos"]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub order: Option<i32>,
}

impl Todo {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Todo>> {
        all_todos.order(todos::order.desc()).load::<Todo>(conn)
    }

    pub fn insert(todo: TodoCreate, conn: &PgConnection) -> QueryResult<Todo> {
        diesel::insert_into(todos::table)
            .values(&todo)
            .get_result::<Todo>(conn)
    }

    pub fn get(id: i32, conn: &PgConnection) -> QueryResult<Todo> {
        all_todos.find(id).first::<Todo>(conn)
    }

    pub fn update(id: i32, todo_update: TodoUpdate, conn: &PgConnection) -> QueryResult<Todo> {
        diesel::update(all_todos.find(id))
            .set(&todo_update)
            .get_result(conn)
    }

    pub fn delete(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(all_todos.find(id)).execute(conn)
    }

    pub fn delete_all(conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(all_todos).execute(conn)
    }
}