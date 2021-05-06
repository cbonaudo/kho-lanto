extern crate config;

mod env;
mod schedule;
mod startup;
mod websocket;

use std::thread;

fn main() {
    startup::startup_actions();

    let schedule_thread = thread::spawn(|| schedule::start_scheduler());
    let websocket_thread = thread::spawn(|| websocket::start_websocket());

    let _ = schedule_thread.join();
    let _ = websocket_thread.join();
}
