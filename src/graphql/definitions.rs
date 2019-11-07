use crate::schemas::schemas::posts;

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
