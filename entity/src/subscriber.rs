use core::fmt;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "subscribers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(index)]
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Subscriptions,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Subscriptions => Entity::has_many(super::subscriptions::Entity).into(),
        }
    }
}

impl Related<super::subscriptions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subscriptions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl fmt::Display for ActiveModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.id.as_ref(), self.email.as_ref())
    }
}

