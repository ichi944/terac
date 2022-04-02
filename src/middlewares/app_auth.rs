use axum::{
    extract::{
        FromRequest,
        RequestParts,
        TypedHeader, Extension,
    },
    headers::Cookie,
    http::{
        StatusCode
    },
    async_trait,
};
use sea_orm::{prelude::*};
use crate::models::session;

pub enum AppAuth {
    FoundCurrentUserId(i32),
    None
}

#[async_trait]
impl<B> FromRequest<B> for AppAuth
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(ref db) = Extension::<DatabaseConnection>::from_request(req)
            .await
            .expect("@user_id_from_session: database connection was not loaded");

        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();
        let session_key = cookie
            .as_ref()
            .and_then(|cookie| cookie.get("axum_session"));
        if session_key.is_none() {
            return Ok(Self::None);
        }

        let session_model = session::Entity::find()
            .filter(session::Column::SessionKey.eq(session_key.unwrap().to_owned()))
            .one(db)
            .await
            .unwrap()
            .unwrap();
        match session_model.user_id {
            Some(user_id) => Ok(Self::FoundCurrentUserId(user_id)),
            None => Ok(Self::None), 
        }
    }
}
