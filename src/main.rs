mod config;
mod handler;
mod infrastructure;
mod model;
mod router;

//非同期ランタイムであるtokioを用いてmain関数を非同期にする
#[tokio::main]
async fn main() {
    //envファイルから読み込み(エラー無視）
    dotenv::dotenv().ok();
    let app = router::create_router();
    //tcpポート3000で待機するリスナーを非同期でバインド、失敗したらクラッシュ
    let listener =
        tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {:?}", listener);
    axum::serve(listener, app).await.unwrap();
}
