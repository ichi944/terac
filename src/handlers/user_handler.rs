use crate::models::user::Entity as User;
use crate::models::result::Entity as ResultEntity;
use crate::models::result::Model as ResultModel;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
// use sea_orm::error::DbErr;
use sea_orm::prelude::*;
// use serde::Serialize;
use serde_json::json;
use serde::Serialize;

pub struct MyErr(String);
pub enum BaseError {
    DbErr(DbErr),
    MyErr(String),
}
impl From<DbErr> for BaseError {
    fn from(inner: DbErr) -> Self {
        BaseError::DbErr(inner)
    }
}
impl From<MyErr> for BaseError {
    fn from(inner: MyErr) -> Self {
        BaseError::MyErr(inner.0)
    }
}
impl IntoResponse for BaseError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
            "error": "base error".to_string(),
        }));
        (StatusCode::NOT_FOUND, body).into_response()
    }
}

#[derive(Serialize)]
pub struct ResUser {
    id: i32,
    email: String,
    results: Vec<ResultModel>,
}
#[derive(Serialize)]
pub struct UsersResponse {
    users: Vec<ResUser>,
}
pub async fn index(
    Extension(ref db): Extension<DatabaseConnection>,
// ) -> Result<Json<Vec<(Model, Vec<ResultModel>)>>, BaseError> {
) -> Result<Json<UsersResponse>, BaseError> {
    let res = User::find().find_with_related(ResultEntity).all(db).await?;
    // Ok(Json(res))
    let json = UsersResponse {
        users: res.iter().map(|v| ResUser {
            id: v.0.id,
            email: v.0.clone().email,
            results: v.1.clone(),
        }).collect()
    };
    Ok(Json(json))
}
