use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Sparkpost {
    key: String,
    token: String,
    url: String,
    version: u8,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Twitter {
    consumer_token: String,
    consumer_secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Braintree {
    merchant_id: String,
    public_key: String,
    private_key: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    tesuto: String,
    database: Database,
    sparkpost: Sparkpost,
    twitter: Twitter,
    braintree: Braintree,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory.");
        let config_dir = base_path.join("config");

        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::from(config_dir.join("default")))
            .add_source(
                File::from(config_dir.join(run_mode)).required(true)
            )
            .add_source(Environment::with_prefix("app").separator("_"))
            // .set_override("database.url", "postgres://")?
            .build()?;

        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("database.url"));

        s.try_deserialize()
    }
}