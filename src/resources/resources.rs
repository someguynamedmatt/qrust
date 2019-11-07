use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use crate::graphql::roots::{Context, Schema};
use actix_web::{web, Error, HttpResponse};
use std::sync::Arc;

pub fn graphiql() -> HttpResponse {
    let html = graphiql_source(dotenv!("GRAPHQL_SOURCE"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub fn graphql(st: web::Data<Arc<Schema>>, ctx: web::Data<Context>, data: web::Json<GraphQLRequest>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .map_err(Error::from)
        .and_then(|user| {
            Ok(HttpResponse::Ok()
               .content_type("application/json")
               .body(user))
        })
}

