use futures_util::FutureExt;

use crate::base::utils::wait_util::wait;

pub async fn timeout<F, T>(future: F, timeout_ms: u32) -> Result<T, String>
where
    F: std::future::Future<Output = T>,
{
    futures::select! {
        res = future.fuse() => Ok(res),
        _ = wait(timeout_ms).fuse() => {
            tracing::error!("timed out");
            Err("timed out".to_string())
        },
    }
}
