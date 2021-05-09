use crate::domain::Startup;

pub async fn startup_actions() {
    Startup::send_start_messages().await;
}
