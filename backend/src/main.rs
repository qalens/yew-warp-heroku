#![deny(warnings)]
use warp::Filter;
use std::*;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}