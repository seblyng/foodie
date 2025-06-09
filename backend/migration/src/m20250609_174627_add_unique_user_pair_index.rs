use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "CREATE UNIQUE INDEX unique_user_pair 
                ON friendships (
                    LEAST(requester_id, recipient_id), 
                    GREATEST(requester_id, recipient_id)
                )",
        )
        .await?;

        db.execute_unprepared(
            "ALTER TABLE friendships ADD CONSTRAINT no_self_request CHECK (requester_id <> recipient_id)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DROP INDEX IF EXISTS unique_user_pair")
            .await?;

        db.execute_unprepared("ALTER TABLE friendships DROP CONSTRAINT IF EXISTS no_self_request")
            .await?;

        Ok(())
    }
}
