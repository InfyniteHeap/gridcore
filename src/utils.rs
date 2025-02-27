use std::sync::LazyLock;
use std::thread;

use tokio::sync::RwLock;

/// The number of threads to use for parallel downloading.
pub(crate) static THREAD_COUNT: LazyLock<RwLock<usize>> =
    LazyLock::new(|| RwLock::new(thread::available_parallelism().unwrap().get()));

/// Set thread count.
///
/// If you don't call this function,
/// the default thread count will depend on
/// numbers of logical CPU cores.
pub async fn set_thread_count(count: usize) {
    *THREAD_COUNT.write().await = count;
}
