use async_std::task;
use job_scheduler::{Job, JobScheduler};

use crate::domain::{Camp, SaveFile};

pub fn start_scheduler() {
    let mut sched = JobScheduler::new();

    // Every second
    sched.add(Job::new("0/5 * * * * * *".parse().unwrap(), || {
        let task = task::spawn(async {
            Camp::display_interface().await;
        });

        task::block_on(task);
    }));

    sched.add(Job::new("0/50 * * * * * *".parse().unwrap(), || {
        SaveFile::save_states()
    }));

    loop {
        sched.tick();
        std::thread::sleep(sched.time_till_next_job());
    }
}
