use directories::ProjectDirs;

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub max_parallel_task: u8,
    pub leader_uri: String,
    pub test_data_download_location: String,
}

impl Config {
    pub fn import(config_path: &String) -> Self {
        let file_content = std::fs::read_to_string(config_path).unwrap();
        let parsed = json::parse(&file_content).unwrap();

        Self {
            max_parallel_task: parsed["max_parallel_task"].as_u8().unwrap(),
            leader_uri: parsed["leader_uri"].as_str().unwrap().to_string(),
            test_data_download_location: parsed["test_data_download_location"].as_str().unwrap().to_string(),
        }
    }
}

pub fn get_path() -> String {
    if let Ok(res) = std::env::var("CFG") {
        return res;
    }

    if let Ok(_) = std::fs::metadata("./judger.config.json") {
        return String::from("./judger.config.json");
    }

    if let Some(proj_dirs) = ProjectDirs::from("org", "SegmentOJ",  "SegmentOJ Judger") {
        return format!("{}/judger.config.json", proj_dirs.config_dir().to_str().unwrap());    
    }

    unreachable!();
}
