use diesel::prelude::*;
use juniper::RootNode;

use crate::db::database::PostgresPool;
use crate::schemas::schemas::posts;

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


#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[juniper::object(description = "A post")]
impl Post {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn body(&self) -> &str {
        self.body.as_str()
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}) 
}
