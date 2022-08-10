use dotenv;

mod prisma;

use prisma::post;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = prisma::new_client().await.unwrap();

    let post = client
        .post()
        .create(
            post::title::set("First post".to_string()),
            vec![],
        )
        .exec()
        .await
        .unwrap();
    println!("{:?}", post);

    let posts = client
        .post()
        .find_many(vec![])
        .exec()
        .await
        .unwrap();
    println!("{:?}", posts);
}
