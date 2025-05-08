//! Experimental implementation of the Embassy HAL for the RA4M1
//! 
//!
#![no_std]

pub mod gpio;

#[cfg(feature = "ra4m1")]
pub use ra4m1_pac as pac;

// Include the generated file to ensure macros like `foreach_pin` are available.
#[path ="../cfg/generated.rs"]
#[macro_use]
pub mod generated;
