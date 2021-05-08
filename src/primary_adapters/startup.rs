use crate::domain::Startup;

pub async fn startup_actions() {
    Startup::delete_all_messages().await;

    Startup::send_start_messages().await;
}
