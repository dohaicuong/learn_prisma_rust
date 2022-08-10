use std::sync::Arc;

use dotenv::dotenv;
use axum::{
    Server,
    Router,
    extract::Extension,
};

mod prisma;
mod routes;

// use prisma::post;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let prisma_client = prisma::new_client().await.unwrap();
    let prisma_client = Arc::new(prisma_client);

    let app = Router::new()
        .nest("/api", routes::create_routes())
        .layer(Extension(prisma_client));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
