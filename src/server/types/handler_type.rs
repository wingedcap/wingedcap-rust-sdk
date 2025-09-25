use std::pin::Pin;

pub type Handler = Box<
    dyn Fn(String, String) -> Pin<Box<dyn Future<Output = Option<Result<String, String>>> + Send>>
        + Send
        + Sync,
>;
