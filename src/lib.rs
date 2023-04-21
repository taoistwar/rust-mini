use std::env;

pub const DATABASE_NAME_ENV: &str = "DATABASE_NAME";
pub const DATABASE_HOST_ENV: &str = "DATABASE_HOST";
pub const DATABASE_USER_ENV: &str = "DATABASE_USER";
pub const DATABASE_PASSWORD_ENV: &str = "DATABASE_PASSWORD";

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
