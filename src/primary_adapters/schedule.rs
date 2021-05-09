extern crate job_scheduler;

use async_std::task;
use job_scheduler::{Job, JobScheduler};
use serde_json::json;

use crate::domain::SaveFile;

pub fn start_scheduler() {
    let mut sched = JobScheduler::new();

    // Every second
    sched.add(Job::new("0/50 * * * * * *".parse().unwrap(), || {
        // send_messages(&token);
    }));

    sched.add(Job::new("0/50 * * * * * *".parse().unwrap(), || {
        save_states();
    }));

    loop {
        sched.tick();
        std::thread::sleep(sched.time_till_next_job());
    }
}

pub fn _send_messages(token: &str) {
    let header = format!("Bot {}", token);

    let task = task::spawn(async {
        let request_url = "https://discord.com/api/channels/839488059253719080/messages";

        let message = "Bonjour, c'est Bronnie Degniart et bienvenue a Kho Lanto !";

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
}

pub fn save_states() {
    SaveFile::save_states()
}
