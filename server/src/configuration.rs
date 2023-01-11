use std::fmt::{Debug, Display};

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub application: ApplicationConfiguration,
}

#[derive(serde::Deserialize)]
pub struct ApplicationConfiguration {
    pub port: u16,
    pub host: String,
}

/// The possible runtime environment for the application.
pub enum Environment {
    Local,
    Production,
}

impl Default for Environment {
    fn default() -> Self {
        Self::Local
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Local => write!(f, "local"),
            Environment::Production => write!(f, "production"),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "`{other}` is not a supported environment; use either `local` or `production`."
            )),
        }
    }
}

/// Loads configuration from *.yaml files stored inside a local `./configuration/` directory.
///
/// Expects:
/// - the configuration files to be stored inside a local `./configuration/` directory
/// - a `base.yaml` configuration file exists, which is used for every environment
/// - the potential value of the `APP_ENVIRONMENT` environment variable to be a valid `ENVIRONMENT` name
/// - a *.yaml configuration for each `ENVIRONMENT` exists
pub fn get_configuration<ENVIRONMENT, CONFIGURATION>() -> Result<CONFIGURATION, config::ConfigError>
where
    ENVIRONMENT: Default + ToString + TryFrom<String>,
    <ENVIRONMENT as TryFrom<String>>::Error: Debug,
    CONFIGURATION: serde::de::DeserializeOwned,
{
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Detect the running environment.
    let environment: ENVIRONMENT = std::env::var("APP_ENVIRONMENT")
        .map(|environment_string| {
            environment_string
                .try_into()
                .expect("Failed to parse APP_ENVIRONMENT")
        })
        .unwrap_or_default();

    let environment_filename = format!("{}.yaml", environment.to_string());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<CONFIGURATION>()
}
