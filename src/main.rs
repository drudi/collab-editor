use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let _app = collab_editor::build_app();

    // Placeholder: no server listening yet
    tracing::info!("collab-editor built successfully");
}
