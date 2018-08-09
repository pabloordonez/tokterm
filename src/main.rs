extern crate tokterm_core;
use tokterm_core::Result;

///////////////////////////////////////////////////
/// Windows Test App
///////////////////////////////////////////////////
#[cfg(windows)]
extern crate tokterm_windows;

#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
use windows::execute;

///////////////////////////////////////////////////
/// unix Test App
///////////////////////////////////////////////////
#[cfg(unix)]
extern crate tokterm_unix;

#[cfg(unix)]
pub mod unix;

#[cfg(unix)]
use unix::execute;

fn main() -> Result<()> {
    execute()?;
    Ok(())
}