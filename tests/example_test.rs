#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod test {
    use defmt_rtt as _;

    #[init]
    fn init() -> () {
        let _peripherals = esp_hal::init(Default::default());
    }

    #[test]
    fn test() {
        defmt::assert_eq!(1 + 1, 2);
    }
}
