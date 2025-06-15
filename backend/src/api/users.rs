use axum::{
    extract::{Query, State},
    Json,
};

use common::user::User;
use futures_util::StreamExt;
use sea_orm::{
    sea_query::{extension::postgres::PgExpr, Expr},
    Condition,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    auth_backend::AuthSession,
    entities::{self, users},
    ApiError,
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
) -> Result<Json<Vec<User>>, ApiError> {
    let user = auth.user.unwrap();

    let search = query.search.to_lowercase();
    let users = entities::users::Entity::find()
        .filter(
            Condition::all()
                .add(
                    Condition::any()
                        .add(Expr::col(users::Column::Name).ilike(format!("%{search}%")))
                        .add(Expr::col(users::Column::Email).ilike(format!("%{search}%"))),
                )
                .add(users::Column::Id.ne(user.id)),
        )
        .stream(&db)
        .await?
        .map(|u| {
            let u = u.unwrap();
            User {
                id: u.id,
                name: u.name,
                email: u.email,
            }
        })
        .collect::<Vec<_>>()
        .await;

    Ok(Json(users))
}
