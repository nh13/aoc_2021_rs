pub mod day0;
pub mod day1a;
pub mod day1b;
pub mod day2a;
pub mod day2b;
pub mod day3a;

use std::error::Error;

use enum_dispatch::enum_dispatch;

pub type DynError = Box<dyn Error>;

#[enum_dispatch]
pub trait CommandImpl {
    fn main(&self) -> Result<(), DynError>;
}
