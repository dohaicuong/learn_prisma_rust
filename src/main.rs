use dotenv::dotenv;
use axum::{
    Server,
    Router,
    routing::get
};
// mod prisma;

// use prisma::post;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(|| async { "Hello world!" }));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
