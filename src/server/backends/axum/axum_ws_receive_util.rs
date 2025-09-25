use axum::extract::ws::WebSocket;

use crate::timeout;

pub async fn ws_receive(socket: &mut WebSocket, timeout_ms: u32) -> Result<String, String> {
    timeout(socket.recv(), timeout_ms)
        .await
        .map_err(|e| {
            let error = format!("connection timed out: {}", e.to_string());
            tracing::error!(error);
            error
        })?
        .ok_or("Failed to receive message from client")?
        .map_err(|e| format!("Failed to receive message from client: {}", e.to_string()))?
        .into_text()
        .map_err(|e| format!("Failed to convert message to text: {}", e.to_string()))
        .map(|m| m.to_string())
}
