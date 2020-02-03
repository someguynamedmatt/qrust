table! {
    people (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        age -> Int4,
        sex -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    people,
    posts,
);
