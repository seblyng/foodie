use common::{
    friendship::FriendshipStatus,
    user::{CreateUser, UserWithRelation},
};
use sqlx::PgPool;

use crate::TestApp;

#[sqlx::test(migrations = false)]
async fn test_get_users_status(pool: PgPool) -> Result<(), anyhow::Error> {
    let app = TestApp::new(pool.clone()).await?;

    let first_user = app
        .create_user(&CreateUser {
            name: "foo".to_string(),
            email: "bar@bar.com".to_string(),
            password: "foo".to_string(),
        })
        .await?;

    app.post::<(), _>(format!("/api/friends/new/{}", first_user.id), None)
        .await?;

    let search = "foo";
    let res = app.get(format!("/api/users?search={}", search)).await?;
    let status = res.json::<Vec<UserWithRelation>>().await?;
    let friendship_one = &status[0];

    assert_eq!(friendship_one.id, first_user.id);
    assert_eq!(friendship_one.name, first_user.name);
    assert_eq!(friendship_one.status, Some(FriendshipStatus::Pending));
    assert_eq!(friendship_one.requester_id, Some(app.user.id));
    assert_eq!(friendship_one.recipient_id, Some(first_user.id));

    Ok(())
}
