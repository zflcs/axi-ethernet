#![doc = "Peripheral access API for AXI-ETHERNET microcontrollers (generated using svd2rust v0.30.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.30.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
use core::marker::PhantomData;
use core::ops::Deref;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 0] = [];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        match self {}
    }
}
#[doc = "AXI Ethernet MAC"]
pub struct AXI_ETHERNET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AXI_ETHERNET {}
impl AXI_ETHERNET {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const axi_ethernet::RegisterBlock = 0x6011_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const axi_ethernet::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for AXI_ETHERNET {
    type Target = axi_ethernet::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AXI_ETHERNET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_ETHERNET").finish()
    }
}
#[doc = "AXI Ethernet MAC"]
pub mod axi_ethernet;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AXI_ETHERNET"]
    pub AXI_ETHERNET: AXI_ETHERNET,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        Peripherals {
            AXI_ETHERNET: AXI_ETHERNET {
                _marker: PhantomData,
            },
        }
    }
}
