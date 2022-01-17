#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System LED controller

use ral_registers::{RWRegister};
use core::marker::PhantomData;

/// Led control register
pub mod LED1 {
    /// Led enable
    pub mod ENABLE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
    
        /// Mask (1 bit: 0x1 << 0)
        pub const mask: u32 = 0x1 << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }
    /// Led color
    pub mod COLOR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
    
        /// Mask (12 bit: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
    
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    
    }}

pub struct RegisterBlock {
    /// Led control register
    pub LED1: RWRegister<u32>,
}

pub struct ResetValues {
    pub LED1: u32,
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
