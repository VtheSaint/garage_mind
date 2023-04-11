use std::net::SocketAddr;
use axum::{response::Html, routing::get, Router};
use dotenv::dotenv;
use std::env;
use std::str::FromStr;

mod adapters;
mod connector;
// mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app: Router = Router::new().route("/", get(root));

    let parsed_address= env::var("SERVER_ADDRESS")
        .unwrap_or_else(|e| panic!("Failed to get env with name 'SERVER_ADDRESS': {:?}", e));

    // TODO: to future
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|e| panic!("Failed to get env with name 'DATABASE_URL': {:?}", e));

    let addr: SocketAddr = SocketAddr::from_str(parsed_address.as_str())
        .unwrap_or_else(|e| panic!("Failed to parse Socket Address: {:?}", e));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html("hello world")
}
