use crate::schemas::schemas::posts;
use crate::schemas::schemas::people;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Person)]
#[table_name="posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub person_id: i32,
}

#[derive(Queryable, Identifiable)]
#[table_name="people"]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub sex: String,
}

#[derive(juniper::GraphQLInputObject, Insertable, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "people"]
pub struct NewPerson {
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub sex: String,
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

    pub fn personId(&self) -> i32 {
        self.person_id
    }
}

#[juniper::object(description = "A person")]
impl Person {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn firstName(&self) -> &str {
        self.first_name.as_str()
    }

    pub fn lastName(&self) -> &str {
        self.last_name.as_str()
    }

    pub fn age(&self) -> i32 {
        self.age
    }

    pub fn sex(&self) -> &str {
        self.sex.as_str()
    }
}
