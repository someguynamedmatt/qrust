#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate juniper;
#[macro_use]
extern crate dotenv_codegen;

use std::io;
use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod db;
mod schemas;
mod graphql;
mod resources;

use crate::db::database::establish_connection;
use crate::graphql::roots::{create_schema, Context};
use crate::resources::resources::{graphiql, graphql};

fn main() -> io::Result<()> {
    dotenv().ok();
    let pool = establish_connection();
    let schema_context = Context { db: pool.clone() };
    let schema = Arc::new(create_schema());
    println!("Running server on {}", dotenv!("HOST"));
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(schema_context.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to_async(graphiql)))
    })
    .bind(dotenv!("HOST"))?
    .run()
}
