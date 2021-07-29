use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

use crate::models::User;

pub struct UserQuery;
pub type UserSchema = Schema<UserQuery, EmptyMutation, EmptySubscription>;

#[Object]
impl User {
    async fn id(&self) -> &i32 {
        &self.id
    }

    async fn name(&self) -> &String {
        &self.name
    }
}

#[Object]
impl UserQuery {
    /// Get all the available users
    async fn get_users(&self, _ctx: &Context<'_>) -> Vec<User> {
        vec![User {
            name: "User name".to_string(),
            id: 13,
        }]
    }
}
