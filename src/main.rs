use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:collab.db".to_string());

    let pool = collab_editor::db::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database at {}", database_url);

    let app = collab_editor::build_app(pool);

    let addr = "0.0.0.0:3000";
    tracing::info!("Listening on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .expect("Failed to bind to port 3000"),
        app,
    )
    .await
    .expect("Server failed");
}
