#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System information and status register

use ral_registers::{RORegister};
use core::marker::PhantomData;

/// System clock frequency in Hertz
pub mod CLOCK_FREQUENCY {
}

/// System clock period in nanoseconds
pub mod CLOCK_PERIOD {
}

pub struct RegisterBlock {
    /// System clock frequency in Hertz
    pub CLOCK_FREQUENCY: RORegister<u32>,

    /// System clock period in nanoseconds
    pub CLOCK_PERIOD: RORegister<u32>,
}

pub struct ResetValues {
    pub CLOCK_FREQUENCY: u32,
    pub CLOCK_PERIOD: u32,
}

pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}

impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
