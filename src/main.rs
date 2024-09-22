use std::fmt::Display;
use std::process::Command;
use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(Counter(1))); // Shared state wrapped in Arc<Mutex<Counter>>

    let app = Router::new().route(
        "/",
        get({
            let counter = Arc::clone(&counter); // Clone the Arc for the handler
            move || async move {
                let mut counter = counter.lock().unwrap(); // Lock the mutex to access the counter
                println!("{counter}. Got request");
                let host =
                    String::from_utf8(Command::new("hostname").arg("-f").output().unwrap().stdout)
                        .unwrap();
                let response = format!("{counter}. Hello, World! from {host:?}\n");
                counter.inc();
                response
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!(">> Server started on port 8080");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug)]
struct Counter(i32);

impl Counter {
    fn inc(&mut self) {
        self.0 += 1;
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
