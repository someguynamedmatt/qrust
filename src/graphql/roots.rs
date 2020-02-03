use diesel::prelude::*;
use juniper::RootNode;

use crate::db::database::PostgresPool;
use crate::schemas::schemas::{people, posts};
use super::definitions::{NewPerson, Person, NewPost, Post};

#[derive(Clone)]
pub struct Context {
    pub db: PostgresPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    fn posts(context: &Context) -> Vec<Post> {
        use crate::schemas::schemas::posts::dsl::*;
        let connection = context.db.get().unwrap();
        posts
            .limit(100)
            .load::<Post>(&connection)
            .expect("[ERROR]: loading posts")
    }

    fn people(context: &Context) -> Vec<Person> {
        use crate::schemas::schemas::people::dsl::*;
        let connection = context.db.get().unwrap();
        people
            .limit(100)
            .load::<Person>(&connection)
            .expect("[ERROR]: loading people")
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_post(context: &Context, data: NewPost) -> Post {
        println!("{:?}", data.title);
        let connection = context.db.get().unwrap();
        diesel::insert_into(posts::table)
            .values(&data)
            .get_result(&connection)
            .expect("[ERROR]: saving new post")
    }

    fn create_person(context: &Context, data: NewPerson) -> Person {
        let connection = context.db.get().unwrap();
        diesel::insert_into(people::table)
            .values(&data)
            .get_result(&connection)
            .expect("[ERROR]: saving new person")
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
