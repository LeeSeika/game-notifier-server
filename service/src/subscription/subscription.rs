use async_nats::jetstream::consumer::push::ConsumerRecreateErrorKind::Subscription;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use sea_orm::ActiveValue::Set;
use entity::subscriptions;
;

pub trait SubscriptionTrait {
    async fn subscribe(&self, uid: i64, player_account_id: Vec<i64>) -> Result<(), DbErr>;
    async fn cancel_subscriptions(&self, uid: i64, player_account_id: Vec<i64>) -> Result<(), DbErr>;
}

#[derive(Clone)]
pub struct SubscriptionService {
    db: &'static DatabaseConnection,
}

impl SubscriptionService {
    pub fn new(db: &'static DatabaseConnection) -> Self {
        Self { db }
    }
}

impl SubscriptionTrait for SubscriptionService {
    async fn subscribe(&self, uid: i64, player_account_id: Vec<i64>) -> Result<(), DbErr> {
        let mut subscription_models: Vec<Subscription::ActiveModel> = Vec::new();
        for id in player_account_id {
            let model = Subscription::ActiveModel {
                uid: Set(uid),
                player_account_id: Set(id),
            };
            subscription_models.push(model);
        }

        subscriptions::Entity::insert_many(subscription_models)
            .exec(self.db)
            .await
            .map(|| ())
    }

    async fn cancel_subscriptions(&self, uid: i64, player_account_id: Vec<i64>) -> Result<(), DbErr> {
        let mut subscription_models: Vec<Subscription::ActiveModel> = Vec::new();
        for id in player_account_id {
            let model = Subscription::ActiveModel {
                uid: Set(uid),
                player_account_id: Set(id),
            };
            subscription_models.push(model);
        }

        let res = subscriptions::Entity::delete_many()
            .exec(self.db)
            .await

    }
}