
use axum::{response::Html, routing::get, Router};

use std::{net::SocketAddr, str::FromStr};

use crate::connector::routes::{Routes, Route};

// use crate::connector::routes::{Route, Routes};

// TODO: Add routes
pub async fn run(parsed_address: String, routes: Routes) {

    let mut app: Router = Router::new();

    for route in routes {
        let Route { path, exec, .. } = route;

        app = app.route(&path, get(move || async move { (exec)() }));
    }

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
