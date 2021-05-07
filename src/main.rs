use std::thread;

mod config;

mod schedule;
mod startup;
mod websocket;

mod models;
mod domain;


#[tokio::main]
async fn main() {

    startup::startup_actions().await;

    let schedule_thread = thread::spawn(|| schedule::start_scheduler());
    let websocket_thread = thread::spawn(|| websocket::start_websocket());

    let _ = schedule_thread.join();
    let _ = websocket_thread.join();
}
