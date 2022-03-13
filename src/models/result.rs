use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "results")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_id: i32,
    pub course_id: i32,
}

#[derive(SimpleObject)]
pub struct CourseResult {
    pub id: i32,
    pub user_id: i32,
    pub course_id: i32,
}
impl From<Model> for CourseResult {
    fn from(from: Model) -> CourseResult {
        CourseResult {
            id: from.id,
            user_id: from.user_id,
            course_id: from.course_id,
        }
    }
}
impl From<ActiveModel> for CourseResult {
    fn from(from: ActiveModel) -> CourseResult {
        CourseResult {
            id: from.id.unwrap(),
            user_id: from.user_id.unwrap(),
            course_id: from.course_id.unwrap(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::UserId)
                .to(super::user::Column::Id)
                .into(),
        }
    }
}
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
