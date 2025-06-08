use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    let routes = Router::new().route("/hello-world", get(hello_world));
    let app = Router::new()
    .nest("/api", routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    print!("Starting a server at port 8080");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> String {
    return "Hello World!".to_string();
}