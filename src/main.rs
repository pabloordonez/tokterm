extern crate tokterm_core;
use tokterm_core::Result;
use tokterm_core::system::application::Application;

#[cfg(windows)]
extern crate tokterm_windows;

#[cfg(unix)]
extern crate tokterm_unix;

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
    use tokterm_unix::application::UnixApplication;
    let mut application = UnixApplication::create()?;
    execute(&mut application)?;
    application.get_terminal().dispose()?;
    Ok(())
}
