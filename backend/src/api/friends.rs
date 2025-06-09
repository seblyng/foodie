use crate::{
    app::AppState,
    auth_backend::AuthSession,
    entities::{
        friendships,
        sea_orm_active_enums::{self, FriendshipStatus},
    },
    storage::FoodieStorage,
    ApiError,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::friendship::FriendshipAnswer;
use sea_orm::{
    sea_query::OnConflict,
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    EntityTrait, IntoActiveModel,
};

// Send a friend request and make it pending.
// If it already exists a friend request/friendship, then do nothing
pub async fn send_friend_request<T>(
    auth: AuthSession,
    Path(recipient_id): Path<i32>,
    State(state): State<AppState<T>>,
) -> Result<impl IntoResponse, ApiError>
where
    T: FoodieStorage + Send + Sync + Clone,
{
    let user = auth.user.unwrap();

    let friendship = friendships::ActiveModel {
        requester_id: Set(user.id),
        recipient_id: Set(recipient_id),
        status: Set(FriendshipStatus::Pending),
        requested_at: Set(chrono::Utc::now().into()),
        responded_at: NotSet,
    };

    friendships::Entity::insert(friendship)
        .on_conflict(
            OnConflict::columns([
                friendships::Column::RequesterId,
                friendships::Column::RecipientId,
            ])
            .do_nothing()
            .to_owned(),
        )
        .exec(&state.db)
        .await?;

    Ok(())
}

// TODO: Should I instead have `accept`, `reject`, `block` endpoints instead of this
// general one?
pub async fn set_friendship_status<T>(
    auth: AuthSession,
    State(state): State<AppState<T>>,
    Json(friendship_answer): Json<FriendshipAnswer>,
) -> Result<impl IntoResponse, ApiError>
where
    T: FoodieStorage + Send + Sync + Clone,
{
    let user = auth.user.unwrap();

    // HACK: I don't like this, but I have a constraint that it cannot save like this
    // recipient_id: 1
    // requester_id: 2
    //
    // recipient_id: 2
    // requester_id: 1
    let id = (
        user.id.min(friendship_answer.user_id),
        user.id.max(friendship_answer.user_id),
    );

    let friendship = friendships::Entity::find_by_id(id).one(&state.db).await?;

    let Some(mut friendship) = friendship.map(|f| f.into_active_model()) else {
        return Err(ApiError::RecordNotFound);
    };

    match friendship.status.as_ref() {
        FriendshipStatus::Accepted => {
            if friendship_answer.status != FriendshipStatus::Blocked.into() {
                return Err(ApiError::BadRequest(
                    "Only possible to block an accepted friendship".to_string(),
                ));
            }
        }
        FriendshipStatus::Pending => {
            if friendship.requester_id.as_ref() == &user.id {
                return Err(ApiError::BadRequest(
                    "Only recipient can accept/reject".to_string(),
                ));
            }
        }
        // TODO(seb): What should be possible to do for a `blocked/rejected` friendship?
        FriendshipStatus::Blocked => todo!("Not implemented yet"),
        FriendshipStatus::Rejected => todo!("Not implemeted yet"),
    }

    friendship.status = Set(friendship_answer.status.into());
    friendship.update(&state.db).await?;

    Ok(())
}

macro_rules! convert_status {
    ($first:ty, $second: ty) => {
        impl From<$first> for $second {
            fn from(value: $first) -> Self {
                match value {
                    <$first>::Pending => <$second>::Pending,
                    <$first>::Accepted => <$second>::Accepted,
                    <$first>::Rejected => <$second>::Rejected,
                    <$first>::Blocked => <$second>::Blocked,
                }
            }
        }
    };
}

convert_status!(
    common::friendship::FriendshipStatus,
    sea_orm_active_enums::FriendshipStatus
);
convert_status!(
    sea_orm_active_enums::FriendshipStatus,
    common::friendship::FriendshipStatus
);
