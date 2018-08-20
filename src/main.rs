extern crate tokterm_core;
use tokterm_core::system::application::Application;
use tokterm_core::Result;

#[cfg(windows)]
extern crate tokterm_windows;

#[cfg(unix)]
extern crate tokterm_termion;
//extern crate tokterm_ncurses;

mod application;
use application::execute;

fn main() -> Result<()> {
    launch()?;
    Ok(())
}

#[cfg(windows)]
fn launch() -> Result<()> {
    use tokterm_windows::application::WindowsApplication;
    let mut application = WindowsApplication::create()?;
    execute(&mut application)?;
    application.get_terminal().dispose()?;
    Ok(())
}

#[cfg(unix)]
fn launch() -> Result<()> {
    use tokterm_termion::application::TermionApplication;
    let mut application = TermionApplication::create()?;
    execute(&mut application)?;
    application.get_terminal().dispose()?;
    Ok(())
}

/*
#[cfg(unix)]
fn launch() -> Result<()> {
    use tokterm_ncurses::application::NCursesApplication;
    let mut application = NCursesApplication::create()?;
    execute(&mut application)?;
    application.get_terminal().dispose()?;
    Ok(())
}
*/