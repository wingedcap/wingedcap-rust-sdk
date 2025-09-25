#[macro_export]
macro_rules! handlers_vec {
    [$(($endpoint:expr, $handler:expr)),* $(,)?] => {{
        use $crate::server::{Handler, register_handler};

        let mut handlers: Vec<Handler> = Vec::new();
        $(
            handlers.push(Box::new(register_handler($endpoint, $handler)));
        )*
        handlers
    }};
}
