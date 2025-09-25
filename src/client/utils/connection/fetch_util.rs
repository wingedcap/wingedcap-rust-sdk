use std::sync::Arc;
use tokio::sync::Mutex;

use tokio_tungstenite_wasm::connect;

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    ReceiveFuture, SecureChannel, SendFuture,
    client::{ws_receive, ws_send},
    types::GenericEndpoint,
};

use crate::client::Server;

pub async fn fetch<I: Serialize, O: std::fmt::Debug>(
    server: &Server,
    endpoint: GenericEndpoint<I, O>,
    payload: &I,
) -> Result<O, String>
where
    O: DeserializeOwned,
{
    let timeout_ms = 10000;

    let url = format!("ws://{}", server.host);

    // Connect to the WebSocket server
    let socket = connect(url).await.map_err(|e| {
        let error = format!("Failed to connect: {}", e.to_string());
        tracing::error!(error);
        error
    })?;

    let shared_socket = Arc::new(Mutex::new(socket));

    let sender_socket = shared_socket.clone();

    let raw_send = move |message: String| {
        let socket = sender_socket.clone();

        let fut = async move {
            let mut socket = socket.lock().await;
            ws_send(&mut socket, &message).await
        };

        Box::pin(fut) as SendFuture
    };

    let receiver_socket = shared_socket.clone();

    let raw_receive = move || {
        let socket = receiver_socket.clone();

        let fut = async move {
            let mut socket = socket.lock().await;
            ws_receive(&mut socket, timeout_ms).await
        };

        Box::pin(fut) as ReceiveFuture
    };

    let channel = SecureChannel::new(raw_send, raw_receive, None, Some(server.pk.clone()))
        .await
        .map_err(|e| {
            let error = format!("Failed to create secure channel: {}", e.to_string());
            tracing::error!(error);
            error
        })?;

    // send endpoint id
    let _ = channel.send(endpoint.id).await.map_err(|e| {
        let error = format!("Failed to send endpoint id: {}", e.to_string());
        tracing::error!(error);
        error
    })?;

    // serialize payload
    let payload_json = serde_json::to_string(&payload).map_err(|e| {
        let error = format!("Failed to serialize payload: {}", e.to_string());
        tracing::error!(error);
        error
    })?;

    // send payload
    let _ = channel.send(&payload_json).await.map_err(|e| {
        let error = format!("Failed to send payload: {}", e.to_string());
        tracing::error!(error);
        error
    })?;

    // receive response
    let response_str = channel.receive().await.map_err(|e| {
        let error = format!("Failed to receive response: {}", e.to_string());
        tracing::error!(error);
        error
    })?;

    // parse response
    let response = serde_json::from_str::<O>(&response_str).map_err(|e| {
        let error = format!("Failed to parse response: {}", e.to_string());
        tracing::error!(error);
        error
    })?;

    Ok(response)
}
