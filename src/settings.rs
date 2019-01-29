use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
  pub server_address: String,
  pub server_port: u16,
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let mut s = Config::new();

    if cfg!(debug_assertions) {
      s.merge(File::with_name("assets/tide-daemon.ini"))?;
    } else {
      s.merge(File::with_name("/usr/local/etc/tide-daemon.ini"))?;
    }

    s.try_into()
  }
}
