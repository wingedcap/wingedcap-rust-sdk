use std::sync::Arc;
use tokio::sync::Mutex;

use axum::extract::ws::WebSocket;

use crate::{
    ReceiveFuture, SecureChannel, SendFuture,
    server::{serve, types::Handler},
};

use super::{ws_receive, ws_send};

pub async fn axum_ws_handler(
    socket: Arc<Mutex<WebSocket>>,
    sk: String,
    handlers: Arc<Vec<Handler>>,
) -> Result<(), String> {
    let timeout_ms = 10000;

    let sender_socket = socket.clone();

    let raw_send = {
        let socket = sender_socket.clone();
        move |message: String| -> SendFuture {
            let socket = socket.clone();
            Box::pin(async move {
                let mut guard = socket.lock().await;
                ws_send(&mut guard, message.to_string()).await
            }) as SendFuture
        }
    };

    let receiver_socket = socket.clone();

    let raw_receive = {
        let socket = receiver_socket.clone();
        move || -> ReceiveFuture {
            let socket = socket.clone();
            Box::pin(async move {
                let mut guard = socket.lock().await;
                ws_receive(&mut guard, timeout_ms).await
            }) as ReceiveFuture
        }
    };

    let channel = SecureChannel::new(raw_send, raw_receive, Some(sk), None)
        .await
        .map_err(|e| {
            let error = format!("Failed to create axum secure channel: {}", e.to_string());
            tracing::error!(error);
            error
        })?;

    serve(handlers, channel).await
}
