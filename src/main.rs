use axum::response;
// use async_graphql::extensions::Extension;
use axum::{
    routing::{get},
    Router,
    response::{IntoResponse},
    extract::{Extension},
    middleware::from_extractor,
};
mod models;
use models::user::User;
use models::result::{CourseResult};
use tower::ServiceBuilder;
use std::net::SocketAddr;
use std::sync::Arc;
// https://www.sheshbabu.com/posts/rust-module-system/
mod handlers;
use handlers::{
    user_handler,
    auth_handler,
    home_handler,
    app_handler,
};
use sea_orm::{prelude::*, ActiveValue::*};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use std::{env};
use async_graphql::{Object, Context};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use openid::DiscoveredClient;
use std::sync::Arc;

use tower_cookies::CookieManagerLayer;

use crate::middlewares::app_auth::{AppAuth};

mod utils;
mod middlewares;

// GraphQL
pub struct Query;

#[Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    async fn users<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<User> {
        let db =ctx.data::<DatabaseConnection>().unwrap();
        let res = models::user::Entity::find().all(db).await.unwrap();
        res.into_iter().map(|v| v.into()).collect::<Vec<User>>()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_result<'ctx>(&self, ctx: &Context<'ctx>, user_id: i32, course_id: i32) -> CourseResult {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let res = models::result::ActiveModel {
            user_id: Set(user_id),
            course_id: Set(course_id),
            ..Default::default()
        }.save(db)
        .await
        .expect("save failed");

        CourseResult::from(res)
    }
}

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;
async fn graphql_handler(
    schema: Extension<ApiSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

// Ref
// https://blog-dry.com/entry/2021/12/26/002649

pub struct AppEnv {
    mode: String,
}

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let mode_string = match &*env::var("MODE").expect("no mode on .env") {
        "development" => "development".to_string(),
        "production" => "production".to_string(),
        _ => panic!("MODE is invalid value on .env"),
    };
    let app_env = Arc::new(AppEnv {
        mode: mode_string,
    });

    // OIDC Client
    let client_id = env::var("OIDC_CLIENT_ID").expect("no client id on .env");
    let client_secret = env::var("OIDC_CLIENT_SECRET").expect("no secret on .env");
    let issuer_url = "https://accounts.google.com".to_string();
    let issuer = reqwest::Url::parse(&issuer_url).unwrap();

    let client = Arc::new(
        DiscoveredClient::discover(
            client_id,
            client_secret,
            Some("".to_string()),
            issuer)
        .await.unwrap(),
        );
    
    // get jwks from "https://www.googleapis.com/oauth2/v3/certs
    let jwks_url = reqwest::Url::parse(&"https://www.googleapis.com/oauth2/v3/certs").unwrap();
    let response = reqwest::get(jwks_url).await.unwrap();
    let jwks = response.text().await.unwrap();
    println!("{}", jwks);
    


    let db_url = env::var("DATABASE_URL").expect("DATABASE is not defined.");
    println!("Trying to connect DB");
    let db = Database::connect(db_url.clone())
    .await
    .expect("Database connection failed");
    println!("Success to connect DB");

    let db2 = Database::connect(db_url.clone())
    .await
    .expect("Database connection failed");

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db2)
        .finish();

    let app = Router::new()
        .route("/app", get(app_handler::index))
        .route_layer(from_extractor::<AppAuth>())
        .route("/", get(home_handler::index))
        .route("/login", get(auth_handler::login))
        .route("/auth/callback", get(auth_handler::callback))
        .route("/logout", get(auth_handler::logout))
        .route("/users", get(user_handler::index))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(app_env))
        .layer(Extension(schema))
        .layer(Extension(client))
        .layer(Extension(jwks))
        .layer(CookieManagerLayer::new())
        .layer(
            ServiceBuilder::new()
                .layer(Extension(db))
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
