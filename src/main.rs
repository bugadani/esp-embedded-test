#![no_std]
#![no_main]

use defmt_rtt as _;
use esp_backtrace as _;

#[esp_hal::entry]
fn main() -> ! {
    loop {}
}
