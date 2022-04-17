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

pub async fn index() -> impl IntoResponse {  
    let template = HomeTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {}

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
