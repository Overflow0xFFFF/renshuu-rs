use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub api_key: Secret<String>,
}

pub fn get() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Settings>()
}
