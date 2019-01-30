#![feature(async_await)]
#[macro_use]
extern crate serde_derive;

mod settings;

use config::ConfigError;
use tide::{configuration::Configuration, App};

fn main() -> Result<(), ConfigError> {
    let settings = crate::settings::Settings::new()?;

    let app_config = Configuration::build()
        .address(settings.server_address.as_ref())
        .port(settings.server_port)
        .finalize();
    let mut app = App::new(());
    app.config(app_config);
    app.at("/").get(async || "Hello World!");
    app.serve();
    Ok(())
}
