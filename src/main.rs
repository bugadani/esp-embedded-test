#![no_std]
#![no_main]

use panic_rtt_target as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let _peripherals = esp_hal::init(Default::default());

    let mut i = 0;
    loop {
        defmt::info!("Tick {=i32}", i);
        i = i.wrapping_add(1);
    }
}
