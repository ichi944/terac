use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use async_graphql::{SimpleObject, ComplexObject, Context};
use crate::models::result::Entity as ResultEntity;
use crate::models::result;
use crate::models::result::CourseResult;


#[derive(Clone,Serialize, Deserialize, SimpleObject)]
pub struct Song {
    pub name: String,
    pub songed_by: String,
}


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub email: String,
}

impl From<Model> for User {
    fn from(from: Model) -> User {
        User { id: from.id, email: from.email }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub email: String,
}


#[ComplexObject]
impl User {
    async fn results<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<CourseResult> {
        let db =ctx.data::<DatabaseConnection>().unwrap();
        let res = ResultEntity::find()
            .filter(result::Column::UserId.eq(self.id)).all(db).await.unwrap();
        res.into_iter()
            .map(|v| { v.into()})
            .collect::<Vec<CourseResult>>()
    }
    async fn songs<'ctx>(&self) -> Vec<Song> {
        let song1 = Song {
            name: "Message of a Rainbow".to_string(),
            songed_by: "Yume, Koharu".to_string(),
        };
        let song3 = Song {
            name: "Von Von Voyage!".to_string(),
            songed_by: "Elza, Kirara".to_string(),
        };
        vec![song1, song3]
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Result,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Result => Entity::has_many(super::result::Entity)
                .into(),
        }
    }
}
impl Related<super::result::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Result.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
