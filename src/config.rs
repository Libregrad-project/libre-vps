use dotenv::dotenv;
use std::env;

pub struct Config {
    pub provider_api: String,
    pub database_url: String,
    pub database_port: u16,
    pub database_user: String,
    pub database_password: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Config {
            provider_api: env::var("PROVIDER_API").expect("PROVIDER_API must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_port: env::var("DATABASE_PORT")
                .expect("DATABASE_PORT must be set")
                .parse()
                .expect("DATABASE_PORT must be a valid number"),
            database_user: env::var("DATABASE_USER").expect("DATABASE_USER must be set"),
            database_password: env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"),
        }
    }
}