mod config;
mod task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::import(&config::get_path());
    let test_task = task::content::Task::fetch(&format!("{}/api/task", config.leader_uri).to_string()).await;

    println!("{:?}", test_task);

    Ok(())
}
