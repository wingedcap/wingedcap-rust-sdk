use axum::extract::ws::{Message, WebSocket};

pub async fn ws_send(socket: &mut WebSocket, message: String) -> Result<(), String> {
    socket
        .send(Message::Text(message.into()))
        .await
        .map_err(|e| format!("failed to send message: {}", e.to_string()))
}
