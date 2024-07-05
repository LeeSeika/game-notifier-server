use sea_orm::DbErr;
use entity::dto::PlayerAccountID;

pub trait UserTrait {
    async fn first_or_create(&self,  email:&str) -> Result<(entity::subscriber::ActiveModel, Vec<PlayerAccountID>), DbErr>;
}

#[derive(Clone)]
pub struct UserService {
    db: &'static sea_orm::DatabaseConnection,
}

impl UserService {
    pub fn new(db: &'static sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}

impl  UserTrait for UserService {
    async fn first_or_create(&self, email: &str) -> Result<(entity::subscriber::ActiveModel, Vec<PlayerAccountID>), DbErr> {
        let user = entity::subscriber::Entity::find()
            .filter(column::email.contains(email))
            .one(self.db)
            .await?;
        if let Some(user) = user {
            let player_accounts = entity::player_account::Entity::find()
                .filter(column::subscriber_id.equals(user.id))
                .all(self.db)
                .await?;
            let player_account_ids = player_accounts.iter().map(|x| PlayerAccountID { id: x.id }).collect();
            Ok((user, player_account_ids))
        } else {
            let user = entity::subscriber::Entity::insert()
                .columns(vec![column::email.eq(email)])
                .exec(self.db)
                .await?;
            Ok((user, vec![]))
        }
    }
}