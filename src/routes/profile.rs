use serde_json::json;

use crate::connector::routes::{RequestType, Route};

pub fn profile() -> Route {
    Route {
        path: String::from("/user"),
        action: RequestType::GET,
        exec
    }
}

fn exec() -> String {
    println!("profile exec");
    json!({"profile" : "output"}).to_string()
}