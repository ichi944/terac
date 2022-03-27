use std::str;
use axum::{
    body::{self, Full},
    response::{
        IntoResponse,
        Response,
        Html,
    },
    http::StatusCode,
    Json,
    extract::{
        Query, Extension,
        // Extension,
    }
};
use askama::Template;
use serde::{Serialize, Deserialize};
use serde_json;
use serde_json::json;
use alcoholic_jwt::{JWKS, Validation, validate, token_kid};
use base64_url;
use sea_orm::{prelude::*, ActiveValue::*, IntoActiveModel};
use tower_cookies::{Cookies, Cookie};
use crate::models::user;
use crate::models::session;
use crate::utils::generate_session_id_string;

pub async fn login() -> impl IntoResponse {
    let template = LoginTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body::boxed(Full::from(format!(
                    "Failed to render template. Error: {}", err
                ))))
                .unwrap()
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid Token")
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    id_token: String,
}

#[derive(Debug, Serialize)]
pub struct CallbackResponse {
    email: String,
}

pub async fn callback(
    payload: Query<TokenPayload>,
    cookies: Cookies,
    Extension(jwks): Extension<String>,
    Extension(ref db): Extension<DatabaseConnection>,
) -> Result<Json<CallbackResponse>, AuthError> {
    let id_token_string  = payload.id_token.to_string();

    let jwks: JWKS = serde_json::from_str(&jwks).unwrap();

    let kid = token_kid(&id_token_string).unwrap().unwrap();

    let jwk = jwks.find(&kid).expect("speficied key not found in set");

    let validations = vec![Validation::Issuer("accounts.google.com".into())];
    validate(&id_token_string, jwk, validations).expect("Token validation has failed!");

    println!("validated");

    
    let v: Vec<&str> = id_token_string.split('.').collect();
    let claim_decoded = base64_url::decode(v[1]).unwrap();
    let s = str::from_utf8(claim_decoded.as_slice()).unwrap();
    println!("claim decoded: {}", s.to_string());
    
    let claim: Claims = serde_json::from_str(s).unwrap();
    println!("email: {}", claim.email);

    let result = user::Entity::find().filter(user::Column::Email.eq(claim.email.to_owned())).one(db).await.unwrap();

    let current_user = match result {
        Some(model) => model.into_active_model(),
        None => user::ActiveModel {
                email: Set(claim.email.to_owned()),
                ..Default::default()
            }
            .save(db)
            .await
            .expect("could not save user instance")
    };

    // current_user.
    let session_id = generate_session_id_string::invoke();
    let _session = session::ActiveModel {
        session_key: Set(session_id.clone()),
        user_id: current_user.id,
        payload: Set("".to_string()),
        last_activity: Set(16231415),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("Could not save session");

    cookies.add(Cookie::new("axum_session", session_id));


    Ok(Json(CallbackResponse {
        email: current_user.email.unwrap(),
    }))
}

#[derive(Deserialize)]
pub struct Claims {
    iss: String,
    azp: String,
    aud: String,
    sub: String,
    email: String,
    email_verified: bool,
    at_hash: String,
    name: String,
    picture: String,
    given_name: String,
    family_name: String,
    locale: String,
    iat: usize,
    exp: usize,
    jti: String,
}
