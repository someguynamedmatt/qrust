use diesel::prelude::*;
use juniper::RootNode;

use crate::db::database::PostgresPool;
use crate::schema::posts;
use super::definitions::{NewPost, Post};

#[derive(Clone)]
pub struct Context {
    pub db: PostgresPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    fn posts(context: &Context) -> Vec<Post> {
        use crate::schema::posts::dsl::*;
        let connection = context.db.get().unwrap();
        posts
            .limit(100)
            .load::<Post>(&connection)
            .expect("[ERROR]: loading posts")
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_post(context: &Context, data: NewPost) -> Post {
        let connection = context.db.get().unwrap();
        diesel::insert_into(posts::table)
            .values(&data)
            .get_result(&connection)
            .expect("[ERROR]: saving new post")
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}) 
}
