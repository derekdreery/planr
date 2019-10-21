
use diesel::{prelude::*, r2d2::{Pool, ConnectionManager}};
use dotenv::dotenv;
use std::env;
use chrono::{DateTime, Utc};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    Pool::new(ConnectionManager::new(database_url)).unwrap()
}

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub project_id: i32,
    pub description: String,
    pub due_date: Option<DateTime<Utc>>,
    pub completed: bool,
}

impl Task {
    /// Get a task with the given ID, if it exists.
    pub fn get_by_id(id: i32) -> QueryResult<Option<Task>> {
        use crate::db_schema::task;
        task::table
        
    }
}
