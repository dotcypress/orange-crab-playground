#![no_std]
#![allow(non_camel_case_types)]

pub extern crate embedded_hal as hal;
pub extern crate nb;

pub use orange_crab_pac::arch::interrupt;
pub use orange_crab_pac::*;

pub mod gpio;
pub mod led;
pub mod prelude;
pub mod serial;
pub mod time;
pub mod timer;

use crate::time::{Hertz, NanoSecond};

pub fn clk_frequency() -> Hertz {
  unsafe { Hertz(read_reg!(crate::system, SYSTEM, CLOCK_FREQUENCY)) }
}

pub fn clk_period() -> NanoSecond {
  unsafe { NanoSecond(read_reg!(crate::system, SYSTEM, CLOCK_PERIOD)) }
}