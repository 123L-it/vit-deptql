mod cli;
mod defaults;
mod graphql;
mod models;
mod resolvers;
mod config;

use log;
use env_logger;
use std::io::Result;
use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<()> {
    let cli_args = cli::parse_cli();
    let host = cli_args.value_of("host").unwrap();
    let port = cli_args.value_of("port").unwrap();
    let address = format!("{host}:{port}", host = host, port = port);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server on port {}", port);
    println!(
        "{}",
        format!(
            "\n -> GraphQL Playground: http://{} \n",
            address,
        )
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(graphql::create_schema()))
            .service(graphql::create_service())
            .wrap(Logger::default())
    })
    .bind(address)?
    .run()
    .await
}
