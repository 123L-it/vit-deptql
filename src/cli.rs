use clap::{App, Arg, ArgMatches};
use dotenv::dotenv;

use crate::defaults;

/// Create the server bin cli taking the values from
/// environment variables or as fallback those from the [defaults]
pub fn parse_cli() -> ArgMatches<'static> {
    dotenv().ok();

    let matches = App::new("Vit GraphQL")
        .arg(
            Arg::with_name("port")
                .short("p")
                .env("PORT")
                .default_value(defaults::PORT),
        )
        .arg(
            Arg::with_name("host")
                .short("h")
                .env("HOST")
                .default_value(defaults::HOST),
        )
        .get_matches();

    matches
}
