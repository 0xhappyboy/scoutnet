use std::io;

use app::{app::App, init};

mod app;

fn main() -> io::Result<()> {
    let mut terminal = init::init()?;
    let app_result = App::new().run(&mut terminal);
    init::restore()?;
    app_result
}
