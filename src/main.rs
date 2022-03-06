use axum::response;
// use async_graphql::extensions::Extension;
use axum::{
    routing::{get},
    Router,
    AddExtensionLayer,
    response::{IntoResponse},
    extract::Extension,
};
use models::user::User;
use tower::ServiceBuilder;
use std::net::SocketAddr;
// https://www.sheshbabu.com/posts/rust-module-system/
mod handlers;
use handlers::{
    user_handler,
    message_handler,
    auth_handler,
};
mod models;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use std::{env};
use async_graphql::{Object, Context};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use openid::DiscoveredClient;
use std::sync::Arc;


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

pub type ApiSchema = Schema<Query, EmptyMutation, EmptySubscription>;
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

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

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

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db2)
        .finish();

    let app = Router::new()
        .route("/", get(message_handler::get))
        .route("/login", get(auth_handler::login))
        .route("/auth/callback", get(auth_handler::callback))
        .route("/users", get(user_handler::index))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema))
        .layer(AddExtensionLayer::new(client))
        .layer(AddExtensionLayer::new(jwks))
        .layer( 
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(db))
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
