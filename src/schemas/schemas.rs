table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Varchar,
    }
}

table! {
    people (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        age -> Int4,
        sex -> Varchar,
    }
}
