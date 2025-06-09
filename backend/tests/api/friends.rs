use backend::entities::{friendships, sea_orm_active_enums::FriendshipStatus};
use common::{
    friendship::FriendshipAnswer,
    user::{CreateUser, UserLogin},
};
use hyper::StatusCode;
use sea_orm::{EntityTrait, PaginatorTrait};
use sqlx::PgPool;

use crate::TestApp;

#[sqlx::test(migrations = false)]
async fn test_send_friendrequest(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;

    let new_user = app
        .create_user(&CreateUser {
            name: "foo".to_string(),
            email: "bar@bar.com".to_string(),
            password: "foo".to_string(),
        })
        .await?;

    app.post::<(), _>(format!("api/friends/new/{}", new_user.id), None)
        .await?;

    let friendship = friendships::Entity::find_by_id((app.user.id, new_user.id))
        .one(&app.pool)
        .await?
        .unwrap();

    assert_eq!(friendship.status, FriendshipStatus::Pending);
    assert_eq!(friendship.requester_id, app.user.id);
    assert_eq!(friendship.recipient_id, new_user.id);

    app.login(&UserLogin {
        email: "bar@bar.com".to_string(),
        password: "foo".to_string(),
    })
    .await;

    app.post(
        "api/friends/status",
        Some(&FriendshipAnswer {
            user_id: app.user.id,
            status: common::friendship::FriendshipStatus::Accepted,
        }),
    )
    .await?;

    let friendship = friendships::Entity::find_by_id((app.user.id, new_user.id))
        .one(&app.pool)
        .await?
        .unwrap();

    assert_eq!(friendship.status, FriendshipStatus::Accepted);
    assert_eq!(friendship.requester_id, app.user.id);
    assert_eq!(friendship.recipient_id, new_user.id);

    Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_friendrequest_both_ways_should_fail(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;

    let new_user = app
        .create_user(&CreateUser {
            name: "foo".to_string(),
            email: "bar@bar.com".to_string(),
            password: "foo".to_string(),
        })
        .await?;

    app.post::<(), _>(format!("api/friends/new/{}", new_user.id), None)
        .await?;

    app.login(&UserLogin {
        email: "bar@bar.com".to_string(),
        password: "foo".to_string(),
    })
    .await;

    app.post::<(), _>(format!("api/friends/new/{}", app.user.id), None)
        .await?;

    let count = friendships::Entity::find().count(&app.pool).await?;

    assert_eq!(1, count);

    Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_friendrequest_requester_cannot_accept(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;

    let new_user = app
        .create_user(&CreateUser {
            name: "foo".to_string(),
            email: "bar@bar.com".to_string(),
            password: "foo".to_string(),
        })
        .await?;

    app.post::<(), _>(format!("api/friends/new/{}", new_user.id), None)
        .await?;

    let res = app
        .post(
            "api/friends/status",
            Some(&FriendshipAnswer {
                user_id: new_user.id,
                status: common::friendship::FriendshipStatus::Accepted,
            }),
        )
        .await?;

    assert_eq!(StatusCode::BAD_REQUEST, res.status());

    let friendship = friendships::Entity::find_by_id((app.user.id, new_user.id))
        .one(&app.pool)
        .await?
        .unwrap();

    assert_eq!(FriendshipStatus::Pending, friendship.status);

    Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_friendrequest_cannot_set_accepted_to_pending(
    pool: PgPool,
) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;

    let new_user = app
        .create_user(&CreateUser {
            name: "foo".to_string(),
            email: "bar@bar.com".to_string(),
            password: "foo".to_string(),
        })
        .await?;

    app.post::<(), _>(format!("api/friends/new/{}", new_user.id), None)
        .await?;

    let friendship = friendships::Entity::find_by_id((app.user.id, new_user.id))
        .one(&app.pool)
        .await?
        .unwrap();

    assert_eq!(friendship.status, FriendshipStatus::Pending);
    assert_eq!(friendship.requester_id, app.user.id);
    assert_eq!(friendship.recipient_id, new_user.id);

    app.login(&UserLogin {
        email: "bar@bar.com".to_string(),
        password: "foo".to_string(),
    })
    .await;

    app.post(
        "api/friends/status",
        Some(&FriendshipAnswer {
            user_id: app.user.id,
            status: common::friendship::FriendshipStatus::Accepted,
        }),
    )
    .await?;

    let res = app
        .post(
            "api/friends/status",
            Some(&FriendshipAnswer {
                user_id: app.user.id,
                status: common::friendship::FriendshipStatus::Pending,
            }),
        )
        .await?;

    assert_eq!(StatusCode::BAD_REQUEST, res.status());

    let friendship = friendships::Entity::find_by_id((app.user.id, new_user.id))
        .one(&app.pool)
        .await?
        .unwrap();

    assert_eq!(friendship.status, FriendshipStatus::Accepted);
    assert_eq!(friendship.requester_id, app.user.id);
    assert_eq!(friendship.recipient_id, new_user.id);

    Ok(())
}
