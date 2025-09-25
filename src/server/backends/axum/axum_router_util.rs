use std::sync::Arc;

use futures::SinkExt;

use tokio::sync::Mutex;

use axum::{Router, extract::ws::WebSocketUpgrade, routing::get};

use crate::server::Handler;

use super::axum_ws_handler;

pub fn axum_router(handlers: Arc<Vec<Handler>>, sk: String) -> Router {
    Router::new().route(
        "/",
        get({
            let handlers = handlers.clone();

            move |wsu: WebSocketUpgrade| {
                async move {
                    wsu.on_upgrade({
                        move |ws| {
                            async move {
                                // Spawn in a way that handles non-Send futures
                                std::thread::spawn(move || {
                                    let rt = tokio::runtime::Runtime::new().unwrap();

                                    rt.block_on(async move {
                                        let shared_socket = Arc::new(Mutex::new(ws));

                                        let _ =
                                            axum_ws_handler(shared_socket.clone(), sk, handlers)
                                                .await;

                                        let _ = shared_socket.lock().await.close().await;
                                    });
                                });
                            }
                        }
                    })
                }
            }
        }),
    )
}
