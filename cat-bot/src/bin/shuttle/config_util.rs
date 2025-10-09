use shuttle_runtime::SecretStore;

pub struct Config {
    pub api_key: String,
    pub delay_in_sec: u64,
    pub bot_token: String,
}

impl From<SecretStore> for Config {
    fn from(value: SecretStore) -> Self {
        let api_key = value.get("THE_API_KEY").expect("No API key found");
        let bot_token = value.get("TELOXIDE_TOKEN").expect("No bot token found");
        let delay_in_sec = {
            let delay = value.get("DELAY_IN_SEC").unwrap_or(String::from("10"));
            delay.parse::<u64>().unwrap()
        };

        Self {
            api_key,
            bot_token,
            delay_in_sec,
        }
    }
}
