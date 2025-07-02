#![no_std]
#![no_main]

esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(test)]
#[embedded_test::tests]
mod test {
    #[init]
    fn init() -> () {
        let _peripherals = esp_hal::init(Default::default());
        rtt_target::rtt_init_defmt!();
    }

    #[test]
    fn test() {
        defmt::assert_eq!(1 + 1, 2);
    }
}
