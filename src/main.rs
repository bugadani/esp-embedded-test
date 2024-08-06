#![no_std]
#![no_main]

use defmt_rtt as _;
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, peripherals::Peripherals, system::SystemControl};

#[esp_hal::entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let delay = Delay::new(&clocks);

    loop {
        defmt::info!("Tick");
        delay.delay_millis(500);
        defmt::info!("Tock");
        delay.delay_millis(500);
    }
}
