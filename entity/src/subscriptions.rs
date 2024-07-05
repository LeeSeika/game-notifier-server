use core::fmt;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter, Related, RelationDef, RelationTrait};
use serde::{Deserialize, Serialize};
use crate::dto::PlayerAccountID;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "subscriptions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(index)]
    pub subscriber_id: i64,
    #[sea_orm(index)]
    pub player_account_id: PlayerAccountID,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    Subscriber,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Subscriber => RelationDef::BelongsTo(super::subscriber::Entity),
        }
    }
}

impl Related<super::subscriber::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subscriber.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl fmt::Display for ActiveModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.id.as_ref(), self.subscriber_id.as_ref(), self.player_account_id.as_ref())
    }
}
