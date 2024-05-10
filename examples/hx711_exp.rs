#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, peripherals, peripherals::Peripherals, prelude::*};
use loadcell::hx711::HX711;
use loadcell::LoadCell;
use esp_hal::gpio::IO;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    esp_println::logger::init_logger_from_env();
    let sck = io.pins.gpio4.into_push_pull_output();
    let dt = io.pins.gpio5.into_floating_input();

    let mut load_sensor = HX711::new(sck, dt, delay);
    load_sensor.tare(16);
    load_sensor.set_scale(1.0);

    let mut led = io.pins.gpio8.into_push_pull_output();

    loop {
        log::info!("Hello world!");
        led.toggle();

        if load_sensor.is_ready() {
            let reading = load_sensor.read_scaled().unwrap();
            // let reading = load_sensor.read().unwrap(); // Use this to calibrate the load cell
            log::info!("Weight: {:.0} g", reading);
            // log::info!("Weight: {} g", reading); // Use this to get all the decimals
        }

        delay.delay(500.millis());
    }
}
