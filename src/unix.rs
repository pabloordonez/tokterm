use tokterm_core::Result;
use tokterm_unix::test;
pub fn execute() -> Result<()> {
    test()?;
    Ok(())
}