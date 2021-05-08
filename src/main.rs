use std::thread;

mod config;

mod models;
mod views;
mod controllers;

#[tokio::main]
async fn main() {
    views::startup::startup_actions().await;

    let schedule_thread = thread::spawn(|| views::schedule::start_scheduler());
    let websocket_thread = thread::spawn(|| views::websocket::start_websocket());

    let _ = schedule_thread.join();
    let _ = websocket_thread.join();
}
