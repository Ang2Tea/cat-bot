use cat_bot::configs::Config;
use shuttle_runtime::SecretStore;

pub(crate) fn to_config(value: SecretStore) -> Config {
    let api_key = value.get("THE_API_KEY").expect("No API key found");
    let db_url = String::from("");
    let bot_token = value.get("TELOXIDE_TOKEN").expect("No bot token found");
    let delay_in_sec = {
        let delay = value.get("DELAY_IN_SEC").unwrap_or(String::from("10"));
        delay.parse::<u64>().unwrap()
    };

    Config::new(api_key, db_url, delay_in_sec, bot_token)
}
