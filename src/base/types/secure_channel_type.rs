use std::{future::Future, pin::Pin, sync::Arc};

use x25519_dalek::{EphemeralSecret, PublicKey};

use crate::{aes_decrypt, aes_encrypt, sign, verify};

pub type SendFuture = Pin<Box<dyn Future<Output = Result<(), String>>>>;
pub type ReceiveFuture = Pin<Box<dyn Future<Output = Result<String, String>>>>;

pub struct SecureChannel {
    pub session_key: String,
    pub send: Arc<dyn Fn(String) -> SendFuture>,
    pub receive: Arc<dyn Fn() -> ReceiveFuture>,
}

impl SecureChannel {
    pub async fn new<R, S>(
        raw_send: S,
        raw_receive: R,
        local_sk: Option<String>,
        remote_pk: Option<String>,
    ) -> Result<Self, String>
    where
        S: Fn(String) -> SendFuture + 'static,
        R: Fn() -> ReceiveFuture + 'static + Clone,
    {
        let session_key = {
            let local_sk = EphemeralSecret::random_from_rng(&mut rand::thread_rng());
            let local_pk = PublicKey::from(&local_sk);
            let local_pk_hex = hex::encode(local_pk.to_bytes());

            let _ = raw_send(local_pk_hex).await?;

            let remote_pk_hex = raw_receive().await?;

            let remote_pk_bytes = hex::decode(&remote_pk_hex).map_err(|e| {
                let error = format!("Failed to decode remote public key hex: {}", e.to_string());
                tracing::error!(error);
                error
            })?;

            let remote_pk_bytes_fixed: [u8; 32] = remote_pk_bytes.try_into().map_err(|_| {
                let error = format!("Failed to convert remote public key to fixed array");
                tracing::error!(error);
                error
            })?;

            let remote_pk = PublicKey::try_from(remote_pk_bytes_fixed).map_err(|_| {
                let error = format!("Failed to decode remote public key bytes");
                tracing::error!(error);
                error
            })?;

            let session_key = local_sk.diffie_hellman(&remote_pk);

            hex::encode(session_key.to_bytes())
        };

        let sender_session_key = session_key.clone();

        let send = Arc::new(move |message: String| {
            let encrypted_message = aes_encrypt(&message, &sender_session_key).map_err(|e| {
                let error = format!("failed to encrypt message: {}", e);
                tracing::error!(error);
                error
            });

            match encrypted_message {
                Ok(encrypted_message) => raw_send(encrypted_message),

                Err(e) => Box::pin(async move { Err(e) }) as SendFuture,
            }
        });

        let receiver_session_key = session_key.clone();

        let receive = Arc::new(move || {
            let raw_receive = raw_receive.clone();
            let session_key = receiver_session_key.clone();

            let fut = async move {
                let encrypted_message_result = raw_receive().await;

                match encrypted_message_result {
                    Ok(encrypted_message) => {
                        aes_decrypt(&encrypted_message, &session_key).map_err(|e| {
                            let error = format!("failed to decrypt message: {}", e);
                            tracing::error!(error);
                            error
                        })
                    }

                    Err(e) => {
                        let error = format!("failed to receive message: {}", e);
                        tracing::error!(error);
                        Err(error)
                    }
                }
            };

            Box::pin(fut) as ReceiveFuture
        });

        let channel = SecureChannel {
            session_key: session_key.clone(),
            send,
            receive,
        };

        let sig = if let Some(sk) = local_sk {
            let sig = sign(&sk, &session_key).map_err(|e| {
                let error = format!("failed to sign session key: {}", e);
                tracing::error!(error);
                error
            })?;

            sig
        } else {
            String::new()
        };

        channel.send(&sig).await.map_err(|e| {
            let error = format!("failed to send signature: {}", e);
            tracing::error!(error);
            error
        })?;

        let sig = channel.receive().await?;

        if let Some(remote_pk) = remote_pk {
            let is_valid = verify(&remote_pk, &session_key, &sig).map_err(|e| {
                let error = format!("failed to verify session key: {}", e);
                tracing::error!(error);
                error
            })?;

            if !is_valid {
                return Err("invalid server signature".to_string());
            }
        }

        Ok(channel)
    }

    pub fn get_key(&self) -> String {
        self.session_key.clone()
    }

    pub async fn send(&self, message: &str) -> Result<(), String> {
        (self.send)(message.to_string()).await
    }

    pub async fn receive(&self) -> Result<String, String> {
        (self.receive)().await
    }
}
