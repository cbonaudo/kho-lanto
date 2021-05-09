use std::thread;

mod config;

mod domain;
mod primary_adapters;
mod secondary_adapters;

#[tokio::main]
async fn main() {
    primary_adapters::startup::startup_actions().await;

    let schedule_thread = thread::spawn(|| primary_adapters::schedule::start_scheduler());
    let websocket_thread = thread::spawn(|| primary_adapters::websocket::start_websocket());

    let _ = schedule_thread.join();
    let _ = websocket_thread.join();
}
