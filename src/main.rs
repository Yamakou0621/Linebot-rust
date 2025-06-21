mod config;
mod handler;
mod router;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let app = router::create_router();

    let listener =
        tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {:?}", listener);
    axum::serve(listener, app).await.unwrap();
}
