#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ra::gpio::{Level, Output};
use embassy_ra::generated::peripherals::*;
use core::time::Duration;
use cortex_m::asm::nop;

use rtt_target::{ rprintln, rtt_init_print };

#[embassy_executor::task]
async fn blinker(mut led: Output<'static>, interval: Duration) {
    loop {
        led.set_high();
        // Simple delay
        for _ in 0..10_000 {
            nop();
        }
        //Timer::after(interval).await;
        led.set_low();
        //Timer::after(interval).await;
        for _ in 0..10_000 {
            nop();
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize RTT for printing
    rtt_init_print!();
    rprintln!("Blinking LED on P1_02!");
    let led = Output::new(P1_2 { _private: () }, Level::Low);
    rprintln!("Success init led");
    let _ = spawner.spawn(blinker(led, Duration::from_millis(300)));
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}
