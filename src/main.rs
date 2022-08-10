use dotenv;

mod prisma;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = prisma::new_client().await.unwrap();

    let posts = client.post().find_many(vec![]).exec().await.unwrap();

    println!("{:?}", posts);

    println!("Hello, world!");
}
