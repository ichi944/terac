use axum::{
    response::{
        Json,
    },
};
use serde::Serialize;
use crate::{middlewares::app_auth::AppAuth};

#[derive(Serialize)]
pub struct HelloResponse {
    is_logged_in: bool,
}

pub async fn index(
    auth: AppAuth,
) -> Json<HelloResponse> {
    let is_logged_in = match auth {
        AppAuth::FoundCurrentUserId(_) => true,
        AppAuth::None => false,
    };
    Json(HelloResponse {
        is_logged_in,
    })
}
