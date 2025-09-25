use std::sync::Arc;

use crate::{SecureChannel, server::types::Handler};

pub async fn serve(handlers: Arc<Vec<Handler>>, channel: SecureChannel) -> Result<(), String> {
    let id = channel.receive().await.map_err(|e| {
        let error = format!("failed to decrypt id: {}", e);
        tracing::error!(error);
        error
    })?;

    let input = channel.receive().await.map_err(|e| {
        let error = format!("failed to decrypt input: {}", e);
        tracing::error!(error);
        error
    })?;

    for handler in handlers.iter() {
        let output = handler(id.clone(), input.clone()).await;

        if let Some(output) = output {
            match output {
                Ok(output) => {
                    channel.send(&output).await?;

                    return Ok(());
                }

                Err(e) => {
                    let error = format!("{} handler failed: {}", id, e.to_string());
                    tracing::error!(error);
                    return Err(error);
                }
            }
        }
    }

    {
        let error = format!("no handler found for {}", id);
        tracing::error!(error);
        Err(error)
    }
}
