#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config"))?;
    settings.try_into()
}
