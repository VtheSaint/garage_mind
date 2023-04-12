
use axum::{response::Html, routing::get, Router};

use std::{net::SocketAddr, str::FromStr};

// use crate::connector::routes::{Route, Routes};

// TODO: Add routes
pub async fn run(parsed_address: String) {

    let app: Router = Router::new().route("/", get(root));

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
