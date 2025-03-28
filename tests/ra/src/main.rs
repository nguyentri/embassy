#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use cortex_m::asm::nop;
use rtt_target::{rprintln, rtt_init_print};
use ra4m1::Peripherals;

#[entry]
fn main() -> ! {
    // Initialize RTT for printing
    rtt_init_print!();
    rprintln!("Blinking LED on P1_02!");

    // Obtain RA4M1 peripherals (unsafe if using `steal()`):
    let dp = unsafe { Peripherals::steal() };

    // --- 1) Set P1_02 as an OUTPUT ---
    // Read the PDR (Data Direction Register)
    let mut pdr_val = dp.PORT1.pdr().read().bits();
    // Set bit 2 => pin 2 as output
    pdr_val |= 1 << 2;
    // Write it back
    unsafe {
        dp.PORT1.pdr().write(|w| w.bits(pdr_val));
    }

    // --- 2) Toggle P1_02 in a loop ---
    let mut led_on = false;

    loop {
        // Flip the LED state
        led_on = !led_on;

        // Read the PODR (Output Data Register)
        let mut podr_val = dp.PORT1.podr().read().bits();
        if led_on {
            // set bit 2 => drives P1_02 high
            podr_val |= 1 << 2;
        } else {
            // clear bit 2 => drives P1_02 low
            podr_val &= !(1 << 2);
        }

        // Write the new output value
        unsafe {
            dp.PORT1.podr().write(|w| w.bits(podr_val));
        }

        rprintln!("LED on P1_02 is now {}", if led_on { "ON" } else { "OFF" });

        // Simple delay
        for _ in 0..10_000 {
            nop();
        }
    }
}
