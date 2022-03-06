use axum::extract::Extension;
use axum::http::StatusCode;
use axum::{Json, response::IntoResponse};
use sea_orm::{prelude::*};
use serde::{Serialize};
use serde_json::json;
// use axum::http::{StatusCode};
use crate::models::message::Entity as Message;


#[derive(Serialize)]
pub struct ResponseJson {
    pub message: String,
    pub tags: Vec<String>,
}

pub struct AppError {}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
            "error": "error".to_string(),
        }));
        (StatusCode::NOT_FOUND, body).into_response()
    }
}
pub async fn get(
    Extension(ref db): Extension<DatabaseConnection>
) -> Result<Json<ResponseJson>, AppError> {
// ) ->  Json<ResponseJson> {
    println!("Hello from index_handler");

    let res = Message::find_by_id(2).one(db).await.unwrap();
    match res {
        Some(v) => Ok(Json(ResponseJson {
            message: v.message.to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        })),
        None => Err(AppError  {}),
        // None => Json(ResponseJson {
        //     message: "no message".to_string(),
        // }),
    }
    // println!("{}", res.unwrap().message.to_string());

    // Json(ResponseJson {
    //     message: "akari".to_string(),
    // })

}
