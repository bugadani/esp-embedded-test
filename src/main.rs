#![no_std]
#![no_main]

use defmt_rtt as _;
use esp_backtrace as _;

defmt::timestamp!("{=u64:us}", esp_hal::time::now().ticks());

#[esp_hal::main]
fn main() -> ! {
    let _peripherals = esp_hal::init(Default::default());

    let mut i = 0;
    loop {
        defmt::info!("Tick {=i32}", i);
        i = i.wrapping_add(1);
    }
}
