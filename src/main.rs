// src/main.rs

#![no_std]
#![no_main]


use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");

    let peripherals = embassy_rp::init(Default::default());
    let mut led = Output::new(peripherals.PIN_15, Level::Low);

    defmt::info!("Initialized.");

    loop {
        defmt::info!("LED on!");
        defmt::info!("Hello World");
        defmt::debug!("Debugging log!");
        led.set_high();
        Timer::after(Duration::from_millis(50)).await;

        defmt::info!("LED off!");
        led.set_low();
        Timer::after(Duration::from_millis(950)).await;
    }
}