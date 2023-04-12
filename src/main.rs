use dotenv::dotenv;
use std::env;


mod adapters;
mod connector;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let parsed_address= env::var("SERVER_ADDRESS")
        .unwrap_or_else(|e| panic!("Failed to get env with name 'SERVER_ADDRESS': {:?}", e));

    // TODO: to future
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|e| panic!("Failed to get env with name 'DATABASE_URL': {:?}", e));

    let routes_list = vec![routes::root()];
    
    connector::server::run(parsed_address, routes_list).await;
}
