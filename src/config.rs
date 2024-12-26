use dotenv::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();
}

pub fn get_env(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set", key))
}
