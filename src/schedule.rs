extern crate job_scheduler;

use job_scheduler::{Job, JobScheduler};
use std::sync::atomic::{AtomicBool, Ordering};
use once_cell::sync::Lazy;
use async_std::{task};
use serde_json::json;

static IS_SENDING_MESSAGE: Lazy<AtomicBool> = Lazy::new(|| {
    AtomicBool::new(false)
});

pub fn start_scheduler(token: String) {
    let mut sched = JobScheduler::new();

    // Every second
    sched.add(Job::new(
        "0/50 * * * * * *".parse().unwrap(),
        || {
            send_messages(&token);
        },
    ));

    loop {
        sched.tick();
        std::thread::sleep(sched.time_till_next_job());
    }
}

pub fn send_messages(token: &str) {
    println!("Hello");

    let already_sending = IS_SENDING_MESSAGE.compare_and_swap(false, true, Ordering::SeqCst);
    
    if !already_sending {

        let header = format!("Bot {}", token);

        let task = task::spawn(async {

            let request_url = "https://discord.com/api/channels/839488059253719080/messages";

            let message = "Bonjour, c'est Bronnie Degniart et bienvenue a Kho Lanto !";

            println!("sending");

            if let Err(e) = surf::post(request_url)
                .body(json!({
                    "content": message,
                }))
                .header("Authorization", header)
                .await
            {
                eprintln!("Couldn't send a request to the discord api: {}", e);
            }
        });

        task::block_on(task);
            
        let _ = IS_SENDING_MESSAGE.compare_and_swap(true, false, Ordering::SeqCst);
    }
}
