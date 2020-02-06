use std::collections::HashMap;
use std::str::FromStr;

use warp::Filter;

#[tokio::main]
async fn main() {
    let harsh = harsh::HarshBuilder::new().init().unwrap();

    let init = warp::path!("init" / String)
        .and(warp::get())
        .and(warp::header("referer"))
        .map(move |hash_id: String, referer: String| {
            let ids = harsh.decode(hash_id).unwrap();

            let referer_uri = warp::http::Uri::from_str(referer.as_str())?;
            let query_str = referer_uri.query().unwrap_or("");
            let mut query: HashMap<String, String> = serde_urlencoded::from_str(query_str).unwrap();

            query.insert("traceable_id".to_string(), ids.get(0).unwrap().to_string());

            println!("{:?}", query);

            warp::http::Response::builder()
                .header("Content-type", "image/png")
                .body(base64::decode("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==").unwrap())
        });

    let routes = init;

    let mut args = std::env::args();
    let port = args.nth(1).unwrap_or("3030".to_string());

    warp::serve(routes)
        .run(([0, 0, 0, 0], port.parse().unwrap()))
        .await
}
