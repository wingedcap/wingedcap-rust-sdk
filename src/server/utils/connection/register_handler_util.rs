use std::{fmt::Debug, pin::Pin};

use crate::types::GenericEndpoint;

pub fn register_handler<I, O, F, Fut>(
    endpoint: GenericEndpoint<I, O>,
    f: F,
) -> impl Fn(String, String) -> Pin<Box<dyn Future<Output = Option<Result<String, String>>> + Send>>
where
    I: serde::de::DeserializeOwned + Debug + Send + 'static,
    O: serde::ser::Serialize + Debug + Send + 'static,
    F: Fn(I) -> Fut + Send + Sync + 'static + Clone,
    Fut: Future<Output = Result<O, String>> + Send + 'static,
{
    move |endpoint_id: String, input_text: String| {
        let f_clone = f.clone();

        Box::pin(async move {
            if endpoint_id != endpoint.id {
                return None;
            }

            let input = serde_json::from_str::<I>(&input_text);

            match input {
                Ok(input) => match f_clone(input).await {
                    Ok(output) => {
                        let output_text = serde_json::to_string(&output)
                            .map_err(|e| format!("failed to serialize output: {}", e.to_string()));

                        match output_text {
                            Ok(output_text) => Some(Ok(output_text)),

                            Err(e) => Some(Err(format!(
                                "failed to serialize output: {}",
                                e.to_string()
                            ))),
                        }
                    }

                    Err(e) => {
                        return Some(Err(format!(
                            "{} handler failed: {}",
                            endpoint_id,
                            e.to_string()
                        )));
                    }
                },

                Err(e) => {
                    return Some(Err(format!(
                        "failed to deserialize input: {}",
                        e.to_string()
                    )));
                }
            }
        })
    }
}
