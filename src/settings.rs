use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server_address: String,
    pub server_port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder();

        // read the local file onyl in development mode
        #[cfg(debug_assertions)]
        let builder = builder.add_source(File::with_name("assets/tide-config.ini"));

        #[cfg(not(debug_assertions))]
        let builder = builder.add_source(File::with_name("/usr/local/etc/tide-config.ini"));

        let config = builder.build()?;
        config.try_deserialize()
    }
}
