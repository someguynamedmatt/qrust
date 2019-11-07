# Rust, Postgres, and GraphQL
> A webserver example

This example uses [Diesel](https://github.com/diesel-rs/diesel) to connect to a Postgres instance. For its webserver component it's using [Actix](https://github.com/actix/actix) and [Actix-web](https://github.com/actix/actix-web).

To get this working you need to create a `.env` file that lives at the same level as the `Cargo.toml` file. _YOU MUST_ have the variables:

```
DATABASE_HOST=localhost
DATABASE_USER=your-postgres-username
DATABASE=qrust
DATABASE_URL="postgres://$DATABASE_USER:$DATABASE_PASSWORD@$DATABASE_HOST/$DATABASE"
PORT=:8080
HOST=localhost$PORT
GRAPHQL_SOURCE="http://$HOST/graphql"
```

Get it going (assuming Rust is already at your fingertips):

1) Get `psql` (Postgres) installed on your machine [by following these docs](https://www.postgresql.org/download/)
1) Install the Diesel CLI: `cargo install diesel_cli`
1) Make sure your `.env` file is ready to go as demonstrated above
1) Run `diesel setup`
1) Run `diesel migration run`
1) Run `cargo build && cargo run`
1) ...your server should now be running on whichever host/port you defined in `.env`
1) Graphiql should be available at your `host:port/graphiql` (e.g. `localhost:8080/graphiql`)
1) Add some fake posts, on `/graphiql` run the mutation:
```
mutation {
  createPost(data: {title:"new post", body: "This is a new post"}) {
    title
    id
  }
}
```
1) Now you can query that same post:
```
query {
  posts {
    title
  }
}
```

