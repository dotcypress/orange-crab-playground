#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System LED controller

pub use super::super::peripherals::rgbled::Instance;
pub use super::super::peripherals::rgbled::{RegisterBlock, ResetValues};
pub use super::super::peripherals::rgbled::{LED1};


/// Access functions for the RGBLED peripheral instance
pub mod RGBLED {
    use super::ResetValues;
    use super::Instance;

    const INSTANCE: Instance = Instance {
        addr: 0xf0040000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RGBLED
    pub const reset: ResetValues = ResetValues {
        LED1: 0x0,
    };

    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RGBLED_TAKEN: bool = false;

    /// Safe access to RGBLED
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Instance> {
        crate::arch::interrupt::free(|_| unsafe {
            if RGBLED_TAKEN {
                None
            } else {
                RGBLED_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RGBLED
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(inst: Instance) {
        crate::arch::interrupt::free(|_| unsafe {
            if RGBLED_TAKEN && inst.addr == INSTANCE.addr {
                RGBLED_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RGBLED
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RGBLED_TAKEN = true;
        INSTANCE
    }

    /// Unsafely obtains an instance of RGBLED
    ///
    /// This will not check if `take()` or `steal()` have already been called
    /// before. It is the caller's responsibility to use the returned instance
    /// in a safe way that does not conflict with other instances.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn conjure() -> Instance {
        INSTANCE
    }
}

/// Raw pointer to RGBLED
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RGBLED: *const RegisterBlock = 0xf0040000 as *const _;
