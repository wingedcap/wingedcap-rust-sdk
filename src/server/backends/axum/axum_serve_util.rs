use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};

use crate::server::{Handler, axum_router};

pub async fn axum_serve(handlers: Vec<Handler>, sk: String, host_port: u16) {
    let host_ip = "0.0.0.0";

    let host_address = format!("{host_ip}:{host_port}");

    let handlers = Arc::new(handlers);

    let router = axum_router(handlers, sk);

    let app = router.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_headers(Any)
            .allow_methods(Any),
    );

    let tcp_listener = tokio::net::TcpListener::bind(host_address)
        .await
        .expect("Failed to create tcp listener");

    axum::serve(tcp_listener, app).await.unwrap();
}
