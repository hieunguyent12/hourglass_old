use std::io;
mod app;

use app::Hourglass;

fn main() -> io::Result<()> {
    let mut hourglass = Hourglass::new();

    let mut terminal = Hourglass::start_tui()?;

    let r = hourglass.run(&mut terminal);
    Hourglass::pause_tui()?;
    r
}
