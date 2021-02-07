#![deny(warnings)]
use warp::Filter;
use std::*;
use std::path::PathBuf;
use warp::path::FullPath;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let application = single_page_application("target/dist");
    let api = warp::path!("api" / "hello" / String).map(|name:String| format!("Hello {}",name));
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    warp::serve(application.or(api)).run(([0, 0, 0, 0], port)).await;
}

fn single_page_application(
    dist_dir: impl Into<PathBuf>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let dist_dir = dist_dir.into();

    let index_fallback = warp::path::full()
        .and(warp::fs::file(dist_dir.join("index.html")))
        .and_then(|p: FullPath, index| async move {
            if p.as_str().starts_with("/api") {
                Err(warp::reject())
            } else {
                Ok(index)
            }
        });
    warp::fs::dir(dist_dir).or(index_fallback)
}