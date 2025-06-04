use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

pub const DEFAULT_TIMEOUT_SEC: u64 = 5;

pub fn execute(shutdown_flag: &Arc<AtomicBool>, timeout: u64) {
    let shutdown_flag_clone = Arc::clone(shutdown_flag);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(timeout)).await;
        shutdown_flag_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        println!("Timeout reached, shutting down...");
    });
}

#[tokio::main]
async fn main() {
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    execute(&shutdown_flag, DEFAULT_TIMEOUT_SEC);
}
