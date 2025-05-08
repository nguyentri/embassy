//! GPIO driver.
#![macro_use]
#![allow(unused)]
use core::convert::Infallible;
use core::hint::unreachable_unchecked;

use critical_section::CriticalSection;

use embassy_hal_internal::{impl_peripheral, Peri, PeripheralType};

use crate::{
    pac::{self, common::{Reg, RW}, Peripherals, RegisterValue},
    foreach_pin,
};

use crate::generated::peripherals::*;

/// Port list
#[derive(Debug, Eq, PartialEq)]
pub enum Port {
    /// Port 0
    PORT0,
    /// Port 1
    #[cfg(feature = "port1")]
    PORT1,
    /// Port 2  
    #[cfg(feature = "port2")]
    PORT2,  
    /// Port 3
    #[cfg(feature = "port3")]
    PORT3,
    /// Port 4
    #[cfg(feature = "port4")]
    PORT4,
    /// Port 5
    #[cfg(feature = "port5")]
    PORT5,
    /// Port 6
    #[cfg(feature = "port6")]
    PORT6,
    /// Port 7
    #[cfg(feature = "port7")]
    PORT7,
    /// Port 8
    #[cfg(feature = "port8")]
    PORT8,
    /// Port 9
    #[cfg(feature = "port9")]
    PORT9,
}

/// Type-erased GPIO pin
pub struct AnyPin {
    pub(crate) pin_port: u8,
}

/// GPIO flexible pin.
///
/// This pin can either be a disconnected, input, or output pin, or both. The level register bit will remain
/// set while not in output mode, so the pin's level will be 'remembered' when it is not in output
/// mode.
pub struct Flex<'d> {
    pub(crate) pin: Peri<'d, AnyPin>,
}

/// Represents a digital input or output level.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Level {
    /// Logical low.
    Low,
    /// Logical high.
    High,
}

/// Represents a digital input or output level
impl From<bool> for Level {
    fn from(val: bool) -> Self {
        match val {
            true => Self::High,
            false => Self::Low,
        }
    }
}

impl From<Level> for bool {
    fn from(level: Level) -> bool {
        match level {
            Level::Low => false,
            Level::High => true,
        }
    }
}

/// Input pull configuration
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Pull {
    /// No pull-up or pull-down
    None,
    /// Pull-up
    Up,
    /// Pull-down
    Down,
}

/// GPIO input driver.
pub struct Input<'d> {
    pin: Flex<'d>,
    pull: Pull,
}

/// GPIO output driver.
pub struct Output<'d> {
    pin: Flex<'d>,
}

/// Interface for a Pin that can be configured by an [Input] or [Output] driver, or converted to an [AnyPin].
#[allow(private_bounds)]
pub trait Pin: PeripheralType + Into<AnyPin> + SealedPin + Sized + 'static {
    /// Number of the pin within the port (0..31)
    #[inline]
    fn pin(&self) -> u8 {
        self._pin()
    }
}

pub(crate) trait SealedPin {
    fn pin_port(&self) -> u8;

    #[inline]
    fn _pin(&self) -> u8 {
        self.pin_port() % 16
    }

    #[inline]
    fn _port(&self) -> u8 {
        self.pin_port() / 16
    }

    /// Set the output as high.
    #[inline]
    fn set_high(&self) {
        let peri = unsafe { crate::pac::Peripherals::steal() };
        let port_num = self._port();
        let port = match port_num {
            0 => &peri.PORT0,
            #[cfg(feature = "port1")]
            1 => &peri.PORT1,
            #[cfg(feature = "port2")]
            2 => &peri.PORT2,
            #[cfg(feature = "port3")]
            3 => &peri.PORT3,
            #[cfg(feature = "port4")]
            4 => &peri.PORT4,
            #[cfg(feature = "port5")]
            5 => &peri.PORT5,
            #[cfg(feature = "port6")]
            6 => &peri.PORT6,
            #[cfg(feature = "port7")]
            7 => &peri.PORT7,
            #[cfg(feature = "port8")]
            8 => &peri.PORT8,
            #[cfg(feature = "port9")]
            9 => &peri.PORT9,
            _ => unsafe { unreachable_unchecked() },
        
        };
        let pin_mask = 1 << self._pin();
        unsafe {
            port.porr().write_raw(pin_mask);
        }
    }

    /// Set the output as low.
    #[inline]
    fn set_low(&self) {
        let peri = unsafe { crate::pac::Peripherals::steal() };
        let port_num = self._port();
        let port = match port_num {
            0 => &peri.PORT0,
            #[cfg(feature = "port1")]
            1 => &peri.PORT1,
            #[cfg(feature = "port2")]
            2 => &peri.PORT2,
            #[cfg(feature = "port3")]
            3 => &peri.PORT3,
            #[cfg(feature = "port4")]
            4 => &peri.PORT4,
            #[cfg(feature = "port5")]
            5 => &peri.PORT5,
            #[cfg(feature = "port6")]
            6 => &peri.PORT6,
            #[cfg(feature = "port7")]
            7 => &peri.PORT7,
            #[cfg(feature = "port8")]
            8 => &peri.PORT8,
            #[cfg(feature = "port9")]
            9 => &peri.PORT9,
            _ => unsafe { core::hint::unreachable_unchecked() },
        };

        // Read current value
        let podr = unsafe { port.podr().read() };
        
        // Create pin mask and clear the bit
        let pin_mask = 1u16 << self._pin();
        let new_value = podr.get_raw() & !pin_mask;
        
        // Write back
        unsafe {
            port.podr().write_raw(new_value);
        }
    }

    /// Read the input level.
    #[inline]
    fn is_high(&self) -> bool {
        // let port = self._port();
        // let pin_mask = 1 << self._pin();
        
        // unsafe {
        //     let port_reg = PortRegister::new(port);
        //     (port_reg.pidr().read().bits() & pin_mask) != 0
        // }
        true
    }

    /// Read the input level.
    #[inline]
    fn is_low(&self) -> bool {
        !self.is_high()
    }

    /// Configure the pin as output.
    #[inline]
    fn set_as_output(&self) {
        // let port = self._port();
        // let pin_mask = 1 << self._pin();
        
        // unsafe {
        //     let port_reg = PortRegister::new(port);
        //     port_reg.pdr().modify(|r, w| w.bits(r.bits() | pin_mask));
        // }

    }

    /// Configure the pin as input.
    #[inline]
    fn set_as_input(&self) {
        // let port = self._port();
        // let pin_mask = !(1 << self._pin());
        
        // unsafe {
        //     let port_reg = PortRegister::new(port);
        //     port_reg.pdr().modify(|r, w| w.bits(r.bits() & pin_mask));
        // }
    }

    /// Configure the pin pull resistor.
    #[inline]
    fn set_pull_mode(&self, pull: Pull) {
        // let port = self._port();
        // let pin = self._pin();
        // let pin_mask = 1 << pin;
        
        // unsafe {
        //     let port_reg = PortRegister::new(port);
            
        //     match pull {
        //         Pull::None => {
        //             // Disable pull-up
        //             port_reg.pcr().modify(|r, w| w.bits(r.bits() & !pin_mask));
        //         },
        //         Pull::Up => {
        //             // Enable pull-up
        //             port_reg.pcr().modify(|r, w| w.bits(r.bits() | pin_mask));
        //         },
        //         Pull::Down => {
        //             // RA4M1 doesn't have pull-down, use NMOS open-drain with low output
        //             // This is a workaround and may not work as expected
        //             port_reg.pcr().modify(|r, w| w.bits(r.bits() & !pin_mask));
        //             // Set as open-drain and drive low
        //             port_reg.podr().modify(|r, w| w.bits(r.bits() & !pin_mask));
        //             port_reg.odsr().modify(|r, w| w.bits(r.bits() | pin_mask));
        //         },
        //     }
        // }
    }
}

impl AnyPin {
    /// Unsafely create an `AnyPin` from a pin+port number.
    ///
    /// `pin_port` is `port_num * 16 + pin_num`, where `port_num` is 0 for port `A`, 1 for port `B`, etc...
    #[inline]
    pub unsafe fn steal(pin_port: u8) -> Peri<'static, Self> {
        Peri::new_unchecked(Self { pin_port })
    }

    #[inline]
    fn _port(&self) -> u8 {
        self.pin_port / 16
    }
}

impl SealedPin for AnyPin {
    #[inline]
    fn pin_port(&self) -> u8 {
        self.pin_port
    }
}

#[macro_export]
macro_rules! gpio_pin {
    ($name:ident, $port_type:ident, $port_num:expr, $pin_num:expr) => {
        impl $name {
            #[inline]
            pub(crate) unsafe fn new() -> Self {
                Self { _private: () }
            }
        }

        impl SealedPin for $name {
            #[inline]
            fn pin_port(&self) -> u8 {
                ($port_num * 16) + $pin_num
            }
        }

        impl Pin for $name {}

        impl From<$name> for AnyPin {
            fn from(val: $name) -> Self {
                Self {
                    pin_port: val.pin_port(),
                }
            }
        }
    }
}

// from `impl_peripheral!` macro in generated.rs
impl_peripheral!(AnyPin);

// Implement the Pin trait for each pin
foreach_pin!(gpio_pin);
