use std::env;

pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        // ensure that environment variables are loaded
        ensure_load_env();
        Self {
            port: try_from_env("PORT")
                .parse()
                .expect("Expected a valid u8 for PORT"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { port: 3000 }
    }
}

fn ensure_load_env() {
    println!("ENV loaded from .env");
    // ok to unwrap, if it fails, we should know
    dotenvy::dotenv().unwrap();
}

fn try_from_env(name: &str) -> String {
    let err = format!("Expected {name} in the environment");
    env::var(name).expect(&err)
}
