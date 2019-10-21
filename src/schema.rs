use juniper::FieldResult;
use crate::db::PgPool;
use chrono::{DateTime, Utc};


pub struct Context {
    pool: PgPool
}

impl Context {
    pub fn new(pool: PgPool) -> Self {
        Context { pool }
    }
}

impl juniper::Context for Context {}

#[derive(juniper::GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(juniper::GraphQLObject)]
/// A task 
struct Task {
    /// Database id
    id: i32,
    /// Parent project id 
    project_id: i32,
    /// The main body of the task
    description: String,
    /// When the task is due, if there is a fixed date
    due_date: Option<DateTime<Utc>>,
    /// Whether the task has been completed
    completed: bool,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    fn task(context: &Context, id: i32) -> FieldResult<Task> {
        Ok(Task{
            id,
            project_id: 0,
            description: "test".into(),
            due_date: None,
            completed: true,
        })
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_human(context: &Context, new_human: NewHuman) -> FieldResult<Task> {
        Ok(Task{
            id: 0,
            project_id: 0,
            description: "test".into(),
            due_date: None,
            completed: true,
        })
    }
}

pub type Schema = juniper::RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
