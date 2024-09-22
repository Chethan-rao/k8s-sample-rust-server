use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|| async {
            println!("Got request");
            format!("Hello, World!\n")
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(">> Server started on port 3000");
    axum::serve(listener, app).await.unwrap();
}
