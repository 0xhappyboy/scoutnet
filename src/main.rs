use std::io;

use app::{app::App, init};

mod app;
mod data;
mod net;
mod task;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = init::init()?;
    let app_result = App::new().run(&mut terminal).await;
    init::restore()?;
    app_result?;
    Ok(())
}
