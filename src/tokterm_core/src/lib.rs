use std::result;

pub type Result<T> = result::Result<T, &'static str>;

pub mod drawing;
pub mod events;
pub mod input;
pub mod system;