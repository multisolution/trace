use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::any()
        .map(|| "Hello world");

    let mut args = std::env::args();
    let port = args.nth(1).unwrap_or("3030".to_string());

    warp::serve(routes)
        .run(([0, 0, 0, 0], port.parse().unwrap()))
        .await
}
