#[allow(unused)]
pub struct Config {
    pub api_key: String,
    pub db_url: String,
    pub delay_in_sec: u64,
    pub bot_token: String,
}

impl Config {
    pub fn init() -> Self {
        let api_key = std::env::var("THE_API_KEY").expect("No API key found");
        let db_url = std::env::var("DATABASE_URL").expect("No database url found");
        let bot_token = std::env::var("TELOXIDE_TOKEN").expect("No bot token found");
        let delay_in_sec = {
            let delay = std::env::var("DELAY_IN_SEC").expect("No delay found");
            delay.parse::<u64>().unwrap()
        };

        Self {
            api_key,
            db_url,
            delay_in_sec,
            bot_token,
        }
    }
}
