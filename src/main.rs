use std::{io, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use juniper::http::GraphQLRequest;

mod schema;

use crate::schema::{create_schema, Schema};

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let response_data = data.execute(&st, &()).await;
    HttpResponse::Ok().json(response_data)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    log::info!("starting HTTP server on port 8080");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .wrap(Cors::permissive()) //If you want to use Apollo Studio you need this
            .wrap(middleware::Logger::default())
    })
    // .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
