mod config;
mod task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = reqwest::get("http://172.32.5.72:3000/api/data/1")
    //     .await?
    //     .text_with_charset("utf-8")
    //     .await?;

    // let parsed = json::parse(&resp).unwrap();

    // println!("{}", parsed["res"]);

    let config = config::Config::import(&config::get_path());
    let test_task = task::Task::fetch(&format!("{}/api/task", config.leader_uri).to_string()).await;

    println!("{:?}", test_task);

    Ok(())
}
