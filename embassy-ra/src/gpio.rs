//! GPIO driver.
#![macro_use]
#![allow(unused)]
use core::convert::Infallible;
use core::hint::unreachable_unchecked;

use critical_section::CriticalSection;

use embassy_hal_internal::{impl_peripheral, Peri, PeripheralType};

use crate::pac::common::{Reg, RW};
use crate::{pac, foreach_pin};

/// Port list
#[derive(Debug, Eq, PartialEq)]
pub enum Port{
    /// Port 0
    Port0,
    /// Port 1
    #[cfg(feature = "port1")]
    Port1,
    /// Port 2  
    #[cfg(feature = "port2")]
    Port2,  
    /// Port 3
    #[cfg(feature = "port3")]
    Port3,
    /// Port 4
    #[cfg(feature = "port4")]
    Port4,
    /// Port 5
    #[cfg(feature = "port5")]
    Port5,
    /// Port 6
    #[cfg(feature = "port6")]
    Port6,
    /// Port 7
    #[cfg(feature = "port7")]
    Port7,
    /// Port 8
    #[cfg(feature = "port8")]
    Port8,
    /// Port 9
    #[cfg(feature = "port9")]
    Port9,
}

/// Type-erased GPIO pin
#[derive(Copy, Clone)]
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

/// GPIO input driver.
pub struct Input<'d> {
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

    /// Port of the pin
    #[inline]
    fn port(&self) -> Port {
        match self.pin_port() / 16 {
            0 => Port::Port0,
            #[cfg(feature = "port1")]
            1 => Port::Port1,
            #[cfg(feature = "port2")]
            2 => Port::Port2,
            #[cfg(feature = "port3")]
            3 => Port::Port3,
            #[cfg(feature = "port4")]
            4 => Port::Port4,
            #[cfg(feature = "port5")]
            5 => Port::Port5,
            #[cfg(feature = "port6")]
            6 => Port::Port6,
            #[cfg(feature = "port7")]
            7 => Port::Port7,
            #[cfg(feature = "port8")]
            8 => Port::Port8,
            #[cfg(feature = "port9")]
            9 => Port::Port9,
            _ => unsafe { unreachable_unchecked() },
        }
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
        //let n = self._pin() as _;
        //self.block().bsrr().write(|w| w.set_bs(n, true));
    }

    /// Set the output as low.
    #[inline]
    fn set_low(&self) {
        //let n = self._pin() as _;
        //self.block().bsrr().write(|w| w.set_br(n, true));
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

    /// Get the GPIO register block for this pin.
    #[cfg(feature = "unstable-pac")]
    #[inline]
    pub fn block(&self) -> gpio::Gpio {
        crate::_generated::gpio_block(self._port() as _)
    }
}

// impl_peripheral!(AnyPin);
// impl Pin for AnyPin {
//     #[cfg(feature = "exti")]
//     type ExtiChannel = crate::exti::AnyChannel;
// }
impl SealedPin for AnyPin {
    #[inline]
    fn pin_port(&self) -> u8 {
        self.pin_port
    }
}

impl PeripheralType for AnyPin {}

//foreach_pin!(
//    ($pin_name:ident, $port_name:ident, $port_num:expr, $pin_num:expr, $exti_ch:ident) => {
//        // impl Pin for peripherals::$pin_name {
//        //     #[cfg(feature = "exti")]
//        //     type ExtiChannel = peripherals::$exti_ch;
//        // }
//        impl SealedPin for peripherals::$pin_name {
//            #[inline]
//            fn pin_port(&self) -> u8 {
//                $port_num * 16 + $pin_num
//            }
//        }
//
//        impl From<peripherals::$pin_name> for AnyPin {
//            fn from(val: peripherals::$pin_name) -> Self {
//                Self {
//                    pin_port: val.pin_port(),
//                }
//            }
//        }
//    };
//);
