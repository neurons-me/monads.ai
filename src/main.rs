// src/main.rs
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Entry point for the local monad daemon.
/// This daemon will run indefinitely until manually stopped.
fn main() {
    // Flag to track daemon status
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Handle CTRL+C gracefully
    ctrlc::set_handler(move || {
        println!("\n🔌 Received termination signal. Shutting down the monad...");
        r.store(false, Ordering::SeqCst);
    }).expect("❌ Failed to set Ctrl-C handler");
    println!("✅ monad started. Listening for local events...");
    // Main daemon loop
    while running.load(Ordering::SeqCst) {
        // Placeholder for future logic
        println!("⏳ Waiting...");
        // Sleep to simulate idle time between cycles
        thread::sleep(Duration::from_secs(5));
    }

    println!("🛑 monad stopped.");
}
/// Local monad daemon using HTTP interface.
/// Cross-platform compatible (Unix and Linux).
/// Responds to /status to confirm it's alive.
use axum::{routing::get, Router};
use std::{net::SocketAddr, sync::Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::signal;

#[tokio::main]
async fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let is_running = running.clone();
    // Route to check daemon status
    let app = Router::new().route("/status", get(|| async { "monad active" }));
    // Bind address (cross-platform localhost)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("✅ monad HTTP listening on http://{}/status", addr);
    // Graceful shutdown handler
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async move {
            signal::ctrl_c().await.expect("❌ Failed to install Ctrl+C handler");
            println!("\n🛑 Shutting down...");
            is_running.store(false, Ordering::SeqCst);
        });

    if let Err(e) = server.await {
        eprintln!("❌ Server error: {}", e);
    }
}