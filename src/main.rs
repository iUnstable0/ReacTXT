use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;
use tokio::fs;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/*file_path", get(serve_dynamic_file));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .and_then(|listener| {
            println!("Listening on: {}", listener.local_addr().unwrap());
            Ok(listener)
        })
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    serve_dynamic_file(Path("".to_string())).await
}

async fn serve_dynamic_file(Path(file_path): Path<String>) -> impl IntoResponse {
    println!("Called!");
    let mut file_path = if file_path.is_empty() || file_path == "/" {
        "main.txt".to_string()
    } else {
        format!("{}.txt", file_path.trim_start_matches('/'))
    };

    file_path = format!("pages/{}", file_path);

    match fs::read_to_string(file_path).await {
        Ok(content) => content.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
