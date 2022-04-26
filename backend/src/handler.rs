use axum::{routing, Router};

use crate::config::CONFIG;

async fn get_data() -> &'static str {
    "Hello, World!"
}

pub fn create_app() -> Router {
    let api = Router::new().route("/data", routing::get(get_data));

    let app = Router::new()
        .nest("/api", api)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .fallback(
            routing::get_service(tower_http::services::ServeDir::new(
                &CONFIG.static_file_directory,
            ))
            .handle_error(|error: std::io::Error| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("{}", error),
                )
            }),
        );

    app
}
