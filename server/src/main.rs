use warp::{Filter, Rejection, Reply};

pub fn files() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::fs::dir("../client/dist")
}

pub fn index() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::fs::file("../client/dist/index.html")
}

#[tokio::main]
async fn main() {
    warp::serve(files().or(index())).run(([127, 0, 0, 1], 9009)).await;
}
