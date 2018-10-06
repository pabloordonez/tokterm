extern crate tokterm_core;
use tokterm_core::Result;

#[cfg(feature = "windows")]
extern crate tokterm_windows;

#[cfg(feature = "termion")]
extern crate tokterm_termion;

#[cfg(all(unix, feature = "ncurses"))]
extern crate tokterm_ncurses;

mod application;
use application::execute;

fn main() -> Result<()> {
    launch()?;
    Ok(())
}

#[cfg(feature = "windows")]
fn launch() -> Result<()> {
    use tokterm_windows::application::WindowsApplication;
    let mut application = WindowsApplication::create()?;
    execute(&mut application)?;
    Ok(())
}

#[cfg(feature = "termion")]
fn launch() -> Result<()> {
    use tokterm_termion::application::TermionApplication;
    let mut application = TermionApplication::create()?;
    execute(&mut application)?;
    Ok(())
}

#[cfg(all(unix, feature = "ncurses"))]
fn launch() -> Result<()> {
    use tokterm_ncurses::application::NCursesApplication;
    let mut application = NCursesApplication::create()?;
    execute(&mut application)?;
    Ok(())
}
