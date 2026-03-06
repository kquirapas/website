use std::{env, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    /// Path where expexted static asset file folders reside.
    pub base_directory: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        // ensure that environment variables are loaded
        ensure_load_env();
        Self {
            port: try_from_env("PORT")
                .parse()
                .expect("expected a valid u8 for PORT"),
            base_directory: try_from_env("BASE_DIR").into(),
        }
    }

    pub fn display(&self) {
        println!("{self:?}");
    }
}

impl Default for Config {
    fn default() -> Self {
        let curr_dir = std::env::current_dir().expect("unexpected error getting current directory");

        Self {
            port: 3000,
            base_directory: curr_dir,
        }
    }
}

fn ensure_load_env() {
    println!("ENV loaded from .env");
    // ok to unwrap, if it fails, we should know
    dotenvy::dotenv().unwrap();
}

fn try_from_env(name: &str) -> String {
    let err = format!("expected {name} in the environment");
    env::var(name).expect(&err)
}
