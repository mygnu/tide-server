use async_std::task;
mod settings;

fn main() -> Result<(), std::io::Error> {
    task::block_on(async {
        let settings = crate::settings::Settings::new().unwrap();

        let server_address = format!("{}:{}", settings.server_address, settings.server_port);

        let mut app = tide::new();

        app.at("/").get(move |_| async move { Ok("Hello World!") });

        app.listen(server_address).await?;

        Ok(())
    })
}
