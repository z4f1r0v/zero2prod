use secrecy::{ExposeSecret, Secret};
use std::convert::{TryFrom, TryInto};
use crate::domain::SubscriberEmail;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub email_client: EmailSettings
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct EmailSettings {
    pub base_url: String,
    pub sender_email: String,
}

impl EmailSettings {
    pub fn sender(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to dermine the current directory.");
    let configuration_directory = base_path.join("configuration");

    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    settings.try_into()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environement. Use either `local` or `production`",
                other
            ))
        }
    }
}
impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}?dbname={}&user={}&password={}",
            self.host,
            self.port,
            self.database_name,
            self.username,
            self.password.expose_secret()
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}?user={}&password={}",
            self.host,
            self.port,
            self.username,
            self.password.expose_secret()
        )
    }
}
