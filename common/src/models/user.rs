use serde::{Deserialize, Serialize};

use crate::friendship::FriendshipStatus;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    // picture: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    // picture: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserWithRelation {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub requester_id: Option<i32>,
    pub recipient_id: Option<i32>,
    pub status: Option<FriendshipStatus>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[cfg(feature = "backend")]
impl axum_login::AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &[0u8; 32]
    }
}
