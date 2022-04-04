use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use reqwest;
use log;

use crate::{models::User, config::AppConfig};

pub struct UserQuery;
pub type UserSchema = Schema<UserQuery, EmptyMutation, EmptySubscription>;

#[Object]
impl User {
    async fn username(&self) -> &String {
        &self.username
    }

    async fn enabled(&self) -> &bool {
        &self.enabled
    }

    async fn account_non_locked(&self) -> &bool {
        &self.account_non_locked
    }
}

#[Object]
impl UserQuery {
    /// Get all the available users
    async fn get_users(&self, ctx: &Context<'_>) -> Vec<User> {
        let config = ctx.data_unchecked::<AppConfig>();
        let endpoint = config.user_api.join("auth/users").unwrap();
        let response = reqwest::get(endpoint.as_str())
            .await
            .unwrap()
            .json::<Vec<User>>()
            .await;

        match response {
            Ok(users) => users,
            Err(err) => {
                log::error!("[User] error getting user {} to {}", err, endpoint);
                vec![]
            },
        }
    }
}
