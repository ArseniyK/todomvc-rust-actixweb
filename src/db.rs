use std::ops::Deref;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use crate::models::{Todo, TodoCreate, TodoUpdate};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;


fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn get_all_todos(pool: &PgPool) -> Result<Vec<Todo>, &'static str> {
    Todo::all(get_conn(pool)?.deref()).map_err(|_| "Error getting tasks")
}

pub fn create_todo(
    new_title: String, new_order: Option<i32>, pool: &PgPool
) -> Result<Todo, &'static str> {
    let new_todo = TodoCreate { title: new_title , order: new_order};
    Todo::insert(new_todo, get_conn(pool)?.deref())
        .map_err(|_| "Error creating todo")
}

pub fn delete_all(pool: &PgPool) -> Result<(), &'static str> {
    Todo::delete_all(get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error deleting all todos")
}

pub fn get(id: i32, pool: &PgPool) -> Result<Todo, &'static str> {
    Todo::get(id,get_conn(pool)?.deref())
        .map_err(|_| "Error getting todo")
}

pub fn update(id: i32, todo_update: TodoUpdate, pool: &PgPool) -> Result<Todo, &'static str> {
    Todo::update(id, todo_update,get_conn(pool)?.deref())
        .map_err(|_| "Error getting todo")
}

pub fn delete(id: i32, pool: &PgPool) -> Result<(), &'static str> {
    Todo::delete(id,get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error getting todo")
}