use futures_util::SinkExt;

use tokio_tungstenite_wasm::WebSocketStream;

pub async fn ws_send(socket: &mut WebSocketStream, message: &str) -> Result<(), String> {
    socket
        .send(message.into())
        .await
        .map_err(|e| format!("failed to send message: {}", e.to_string()))
}
