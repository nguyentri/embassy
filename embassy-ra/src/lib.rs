//! Experimental implementation of the Embassy HAL for the RA4M1
//! 
//!
#![no_std]

#[cfg(feature = "ra4m1")]
pub use ra4m1_pac as pac;

// Include generated code
#[macro_use]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}
// publish modules
pub mod gpio;
