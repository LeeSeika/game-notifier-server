use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Statement};
use sea_orm::ActiveValue::Set;
use entity::subscriptions;

pub trait SubscriptionTrait {
    async fn subscribe(&self, uid: i64, player_account_id: Vec<i64>) -> Result<(), DbErr>;
    async fn cancel_subscriptions(&self, uid: i64, player_account_id: Vec<i64>) -> Result<(), DbErr>;
    async fn get_subscribers(&self, player_account_id: i64) -> Result<Vec<i64>, DbErr>;
    async fn get_subscriptions(&self, uid: i64) -> Result<Vec<i64>, DbErr>;
}

#[derive(Clone)]
pub struct SubscriptionService {
    db:  DatabaseConnection,
}

impl SubscriptionService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl SubscriptionTrait for SubscriptionService {
    async fn subscribe(&self, uid: i64, player_account_ids: Vec<i64>) -> Result<(), DbErr> {
        let mut subscription_models: Vec<subscriptions::ActiveModel> = Vec::new();
        for id in player_account_ids {
            let model = subscriptions::ActiveModel {
                id: Default::default(), // NotSet
                subscriber_id: Set(uid),
                player_account_id: Set(id),
            };
            subscription_models.push(model);
        }

        subscriptions::Entity::insert_many(subscription_models)
            .exec(&self.db)
            .await?;

        Ok(())
    }

    async fn cancel_subscriptions(&self, uid: i64, player_account_ids: Vec<i64>) -> Result<(), DbErr> {
        for player_account_id in player_account_ids {
            let res = self.db
                .execute(
                    Statement::from_string(
                        DatabaseBackend::Postgres,
                        format!(
                            "DELETE FROM subscriptions WHERE subscriber_id = {} AND player_account_id = {}",
                            uid, player_account_id
                        ),
                    )
                )
                .await;
            match res {
                Ok(_) => {}
                Err(e) => { eprintln!("error deleting subscription: {}", e); }
            }
        }

        Ok(())
    }

    async fn get_subscribers(&self, player_account_id: i64) -> Result<Vec<i64>, DbErr> {
        let subscriptions = subscriptions::Entity::find()
            .filter(subscriptions::Column::PlayerAccountId.eq(player_account_id))
            .all(&self.db)
            .await?;

        let mut subscribers: Vec<i64> = Vec::new();
        for subscription in subscriptions {
            subscribers.push(subscription.subscriber_id);
        }

        Ok(subscribers)
    }

    async fn get_subscriptions(&self, uid: i64) -> Result<Vec<i64>, DbErr> {
        let subscriptions = subscriptions::Entity::find()
            .filter(subscriptions::Column::SubscriberId.eq(uid))
            .all(&self.db)
            .await?;

        let mut player_account_ids: Vec<i64> = Vec::new();
        for subscription in subscriptions {
            player_account_ids.push(subscription.player_account_id);
        }

        Ok(player_account_ids)
    }
}