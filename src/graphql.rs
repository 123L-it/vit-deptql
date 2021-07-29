use actix_web::{web, HttpRequest, HttpResponse, Resource};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};

use crate::resolvers::user;

/// Create actix service with the GraphQL necessary paths
/// such as the root POST endpoint to receive the schema
/// and the playground on development mode
pub fn create_service() -> Resource {
    web::resource("/")
        .route(web::post().to(post_schema))
        .route(web::get().to(get_playground))
}

/// Create GraphQL schema required by actix
pub fn create_schema() -> Schema<user::UserQuery, EmptyMutation, EmptySubscription> {
    Schema::build(user::UserQuery, EmptyMutation, EmptySubscription).finish()
}

async fn post_schema(
    schema: web::Data<user::UserSchema>,
    _: HttpRequest,
    req: Request,
) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn get_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}
