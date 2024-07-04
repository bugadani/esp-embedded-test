#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod test {
    use defmt_rtt as _;
    use esp_hal::{clock::ClockControl, peripherals::Peripherals, system::SystemControl};

    #[init]
    fn init() -> () {
        let peripherals = Peripherals::take();
        let system = SystemControl::new(peripherals.SYSTEM);
        let _clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    }

    #[test]
    fn test() {
        assert_eq!(1 + 1, 2);
    }
}
