use std::str::FromStr;

use chrono::{DateTime, Utc};
use kernel::model::{id::UserId, role::Role, user::User};
use shared::error::AppError;

pub struct UserRow {
    pub user_id: UserId,
    pub name: String,
    pub email: String,
    pub role_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<UserRow> for User {
    type Error = AppError;
    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        let UserRow {
            user_id,
            name,
            email,
            role_name,
            ..
        } = value;

        Ok(User {
            id: user_id,
            name,
            email,
            role: Role::from_str(role_name.as_str())
                .map_err(|e| AppError::ConversationEntityError(e.to_string()))?,
        })
    }
}
