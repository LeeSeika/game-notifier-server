use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use entity::{subscriber, subscriptions};

pub trait UserTrait {
    async fn first_or_create(&self, email: &str) -> Result<(subscriber::Model, Vec<i64>), DbErr>;
}

#[derive(Clone)]
pub struct UserService {
    db:  sea_orm::DatabaseConnection,
}

impl UserService {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}

impl UserTrait for UserService {
    async fn first_or_create(&self, email: &str) -> Result<(subscriber::Model, Vec<i64>), DbErr> {
        let user = subscriber::Entity::find()
            .filter(subscriber::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        if let Some(user) = user {
            let subscribed_player_ids = subscriptions::Entity::find()
                .filter(subscriptions::Column::SubscriberId.eq(user.id))
                .all(&self.db)
                .await?;
            let player_account_ids = subscribed_player_ids
                .iter()
                .map(|x| x.id )
                .collect();
            Ok((user, player_account_ids))
        } else {
            let new_subscriber = subscriber::ActiveModel {
                email: Set(email.to_string()),
                ..Default::default()
            };

            let uid = subscriber::Entity::insert(new_subscriber)
                .exec(&self.db)
                .await?
                .last_insert_id;

            let new_subscriber = subscriber::Entity::find_by_id(uid)
                .one(&self.db)
                .await?
                .ok_or(DbErr::RecordNotFound(String::from("")))?;

            Ok((new_subscriber, vec![]))
        }
    }
}