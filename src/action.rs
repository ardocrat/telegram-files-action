use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

pub struct Action {
    pub token: String,
    pub chat_ids: Vec<i64>,
    pub files: Vec<PathBuf>,
    pub message: String,
    pub api_url: String,
    pub pin: bool,
    pub delay: Duration,
}

impl Action {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let token = get_env_var("INPUT_TOKEN")?;
        assert_eq!(token.is_empty(), false, "Invalid token");

        let chat_ids: Vec<i64> = get_env_var("INPUT_CHAT_IDS")?
            .split("\n")
            .filter(|s| !s.is_empty())
            .filter(|s| i64::from_str(s).is_ok())
            .map(|s| i64::from_str(s).unwrap())
            .collect();
        assert_eq!(chat_ids.is_empty(), false, "Chat IDs cannot be empty");

        let files: Vec<PathBuf> = get_env_var("INPUT_FILES")?
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| Path::new(s))
            .filter(|p| p.exists() && p.is_file())
            .map(|p| p.to_path_buf())
            .collect();
        assert_eq!(files.is_empty(), false, "Files cannot be empty");

        let message = get_env_var("INPUT_BODY").ok()
            .unwrap_or("".to_string());
        let api_url = get_env_var("INPUT_API_URL").ok()
            .filter(|s| !s.is_empty())
            .unwrap_or(String::from("https://api.telegram.org"));
        let pin = get_env_var("INPUT_PIN").ok()
            .map(|p| p.parse::<bool>().unwrap_or(false))
            .unwrap_or(false);
        let delay = get_env_var("INPUT_DELAY").ok()
            .map(|p| p.parse::<u64>().unwrap_or(5))
            .map(|d| Duration::from_secs(d))
            .unwrap_or(Duration::from_secs(5));

        Ok(Action {
            chat_ids,
            message,
            files,
            token,
            api_url,
            pin,
            delay,
        })
    }
}

fn get_env_var(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("Missing required {}", name))
}