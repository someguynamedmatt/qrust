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

