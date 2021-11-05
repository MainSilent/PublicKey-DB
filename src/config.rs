use serde_json;
use std::fs::File;

pub fn config(key: &str) -> String {
    let config_file = File::open("config.json").expect("Faild to open config file");
    let config: serde_json::Value = serde_json::from_reader(config_file).expect("Faild to parse config file");

    config.get(key).expect("Faild to get key").as_str().unwrap().to_string()
}