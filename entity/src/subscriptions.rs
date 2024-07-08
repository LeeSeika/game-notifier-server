use core::fmt;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "subscriptions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(index)]
    pub subscriber_id: i64,
    #[sea_orm(index)]
    pub player_account_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Subscriber,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Subscriber => Entity::belongs_to(super::subscriber::Entity)
                .from(Column::SubscriberId)
                .to(super::subscriber::Column::Id)
                .into(),
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
