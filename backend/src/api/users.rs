use axum::{
    extract::{Query, State},
    Json,
};

use sea_orm::DatabaseConnection;
use sea_orm::{ConnectionTrait, DbErr, FromQueryResult, Statement};
use serde::Deserialize;

use crate::{
    auth_backend::AuthSession, entities::sea_orm_active_enums::FriendshipStatus, ApiError,
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UserQuery {
    search: String,
}

pub async fn get_users(
    auth: AuthSession,
    State(db): State<DatabaseConnection>,
    Query(query): Query<UserQuery>,
) -> Result<Json<Vec<common::user::UserWithRelation>>, ApiError> {
    let user = auth.user.unwrap();
    let users = fetch_user_relationships(&db, user.id, &query.search).await?;
    Ok(Json(users))
}

pub async fn fetch_user_relationships<C>(
    db: &C,
    id: i32,
    search: &str,
) -> Result<Vec<common::user::UserWithRelation>, DbErr>
where
    C: ConnectionTrait,
{
    let search = format!("%{}%", search.to_lowercase());

    let sql_query = r#"
        SELECT
            u.id,
            u.name,
            u.email,
            f.requester_id,
            f.recipient_id,
            CAST(f.status AS text)
        FROM users u
        LEFT JOIN friendships f
          ON (f.requester_id = $1 AND f.recipient_id = u.id)
          OR (f.recipient_id = $1 AND f.requester_id = u.id)
        WHERE u.id != $1 AND (u.name ILIKE $2 OR u.email ILIKE $2);
    "#;

    let users = UserWithRelation::find_by_statement(Statement::from_sql_and_values(
        db.get_database_backend(),
        sql_query,
        vec![id.into(), search.into()],
    ))
    .all(db)
    .await?;

    let users = users.into_iter().map(|it| it.into()).collect::<Vec<_>>();
    Ok(users)
}

#[derive(Debug, FromQueryResult)]
pub struct UserWithRelation {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub requester_id: Option<i32>,
    pub recipient_id: Option<i32>,
    pub status: Option<FriendshipStatus>,
}

impl From<UserWithRelation> for common::user::UserWithRelation {
    fn from(value: UserWithRelation) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            requester_id: value.requester_id,
            recipient_id: value.recipient_id,
            status: value.status.map(|it| it.into()),
        }
    }
}
