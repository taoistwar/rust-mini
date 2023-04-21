use std::env;

use cornucopia::{CodegenSettings, Error};
use postgres::{Client, NoTls};
pub const DATABASE_NAME_ENV: &str = "DATABASE_NAME";
pub const DATABASE_HOST_ENV: &str = "DATABASE_HOST";
pub const DATABASE_USER_ENV: &str = "DATABASE_USER";
pub const DATABASE_PASSWORD_ENV: &str = "DATABASE_PASSWORD";
fn main() -> Result<(), Error> {
    let queries_path = "queries";
    let destination = format!("{}/controller-sql.rs", std::env::var("OUT_DIR").unwrap());
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: false,
    };

    println!("cargo:rerun-if-changed={queries_path}");

    let config = DatabaseConfig::load();
    let mut client = Client::configure()
        .dbname(&config.name)
        .host(&config.host)
        .user(&config.user)
        .password(&config.password)
        .connect(NoTls)
        .unwrap_or_else(|_| panic!("Could not connect to postgres: {:?}", config));

    cornucopia::generate_live(&mut client, queries_path, Some(&destination), settings)?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub name: String,
    pub host: String,
    pub user: String,
    pub password: String,
}

impl DatabaseConfig {
    pub fn load() -> Self {
        DatabaseConfig {
            name: env::var(DATABASE_NAME_ENV).unwrap_or_else(|_| "arroyo".to_string()),
            host: env::var(DATABASE_HOST_ENV).unwrap_or_else(|_| "172.16.208.114".to_string()),
            user: env::var(DATABASE_USER_ENV).unwrap_or_else(|_| "arroyo".to_string()),
            password: env::var(DATABASE_PASSWORD_ENV).unwrap_or_else(|_| "arroyo".to_string()),
        }
    }
}
