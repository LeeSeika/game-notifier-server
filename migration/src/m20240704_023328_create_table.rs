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
                    .table(Subscribers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subscribers::ID)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Subscribers::Email).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Subscriptions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Subscriptions::ID)
                             .big_integer()
                             .not_null()
                             .primary_key()
                             .auto_increment(), )
                    .col(ColumnDef::new(Subscriptions::SubscriberID)
                        .big_integer()
                        .not_null())
                    .col(ColumnDef::new(Subscriptions::PlayerAccountID)
                        .big_integer()
                        .not_null())
                    .to_owned(),
            )
            .await
    }

    // async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    //     // Replace the sample below with your own migration scripts
    //
    //     // manager
    //     //     .drop_table(Table::drop().table(Subscriber::Table).to_owned())
    //     //     .await
    //     Ok(())
    // }
}

#[derive(DeriveIden)]
enum Subscribers {
    Table,
    ID,
    Email,
}

#[derive(DeriveIden)]
enum Subscriptions {
    Table,
    ID,
    SubscriberID,
    PlayerAccountID,
}
