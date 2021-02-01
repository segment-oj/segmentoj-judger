mod config;
mod task;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::import(&config::get_path());
    let test_task = task::content::Task::fetch(&format!("{}/api/task", config.leader_uri).to_string()).await;

    println!("{}", task::run::fetch_data(test_task, config.test_data_download_location).await);

    Ok(())
}
