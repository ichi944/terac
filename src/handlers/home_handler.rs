use axum::{
    body::{self, Full},
    response::{
        IntoResponse,
        Response,
        Html,
    },
    http::StatusCode,
};
use askama::Template;
use crate::middlewares::app_auth::AppAuth;

pub async fn index(
    auth: AppAuth,
    ) -> impl IntoResponse {
    // let session_id: String = cookies
    //     .get("axum_session")
    //     .and_then(|c| c.value().parse().ok())
    //     .unwrap_or("".to_string());
    
    let is_logged_in = match auth {
        AppAuth::FoundCurrentUserId(_) => true,
        AppAuth::None => false,
    };
    
    let template = HomeTemplate { is_logged_in };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    is_logged_in: bool,
}

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
