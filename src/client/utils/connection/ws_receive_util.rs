use futures_util::StreamExt;

use tokio_tungstenite_wasm::{Message, WebSocketStream};

use crate::base::timeout_util::timeout;

pub async fn ws_receive(socket: &mut WebSocketStream, timeout_ms: u32) -> Result<String, String> {
    let message = timeout(socket.next(), timeout_ms)
        .await
        .map_err(|e| {
            let error = format!("connection timed out: {}", e.to_string());
            tracing::error!(error);
            error
        })?
        .ok_or("Failed to receive message from server")?
        .map_err(|e| format!("Failed to receive message from client: {}", e.to_string()))?;

    match message {
        Message::Text(message_text) => Ok(message_text.to_string()),

        Message::Close(close) => {
            let error = format!("Connection closed by server: {:?}", close);
            tracing::error!(error);
            Err(error)
        }

        _ => {
            let error = format!("Received unexpected message type: {:?}", message);
            tracing::error!(error);
            Err(error)
        }
    }
}
