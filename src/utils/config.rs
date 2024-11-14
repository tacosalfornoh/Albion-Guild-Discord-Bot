use std::env;

pub fn _get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("Expected {} in the environment", key))
}