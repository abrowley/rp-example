// src/main.rs

#![no_std]
#![no_main]


use core::{cmp::Ordering, sync::atomic::AtomicU32};

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pin, Pull};
use embassy_time::{Duration, Timer};
use panic_probe as _;

static SHARED: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task(pool_size =2)]
async fn blink_led(pin: AnyPin){
    let mut led = Output::new(pin, Level::Low);
    loop {
        defmt::info!("LED on");
        let shared_var = SHARED.load(core::sync::atomic::Ordering::Relaxed);
        defmt::info!("Shared var is {:?}",shared_var);
        led.set_high();
        Timer::after(Duration::from_millis(1000)).await;
        defmt::info!("LED off");
        led.set_low();
        Timer::after(Duration::from_millis(1000)).await;
    }
}


#[embassy_executor::task()]
async fn read_button(pin: AnyPin){
    let mut button = Input::new(pin, Pull::Up);
    loop{
        let shared_var = SHARED.load(core::sync::atomic::Ordering::Relaxed);
        button.wait_for_low().await;
        defmt::info!("Button pressed!");
        SHARED.store(shared_var.wrapping_sub(1), core::sync::atomic::Ordering::Relaxed);
        button.wait_for_high().await;
        defmt::info!("Button released");      
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");

    let peripherals = embassy_rp::init(Default::default());
    defmt::info!("Initialized.");
    _spawner.spawn(blink_led(peripherals.PIN_15.degrade())).unwrap();  
    Timer::after(Duration::from_millis(1000)).await;
    _spawner.spawn(blink_led(peripherals.PIN_14.degrade())).unwrap();
    _spawner.spawn(read_button(peripherals.PIN_13.degrade())).unwrap();

    loop {
        defmt::info!("Main loop");
        Timer::after(Duration::from_millis(1000)).await;
    }
}