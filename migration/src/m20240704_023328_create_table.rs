use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Subscriber::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subscriber::ID)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Subscriber::Email).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Subscription::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Subscription::ID)
                             .big_integer()
                             .not_null()
                             .primary_key()
                             .auto_increment(), )
                    .col(ColumnDef::new(Subscription::SubscriberID)
                        .big_integer()
                        .not_null())
                    .col(ColumnDef::new(Subscription::PlayerAccountID)
                        .big_integer()
                        .not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        // manager
        //     .drop_table(Table::drop().table(Subscriber::Table).to_owned())
        //     .await
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Subscriber {
    Table,
    ID,
    Email,
}

#[derive(DeriveIden)]
enum Subscription {
    Table,
    ID,
    SubscriberID,
    PlayerAccountID,
}
