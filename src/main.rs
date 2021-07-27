mod cli;
mod defaults;
mod graphql;
mod models;
mod resolvers;

use std::io::Result;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> Result<()> {
    let cli_args = cli::parse_cli();
    let host = cli_args.value_of("host").unwrap();
    let port = cli_args.value_of("port").unwrap();
    let address = format!("{host}:{port}", host = host, port = port);

    println!(
        "{}",
        format!(
            "\n -> Playground: http://{} \n",
            address,
        )
    );

    HttpServer::new(move || {
        App::new()
            .service(graphql::create_service())
            .data(graphql::create_schema())
    })
    .bind(address)?
    .run()
    .await
}
