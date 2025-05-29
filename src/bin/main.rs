#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::timer::systimer::SystemTimer;
use nb;
use {esp_backtrace as _, esp_println as _}; // Import nb crate for nb::block!

extern crate alloc;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    // let timer1 = TimerGroup::new(peripherals.TIMG0);
    // let _init = esp_wifi::init(
    //     timer1.timer0,
    //     esp_hal::rng::Rng::new(peripherals.RNG),
    //     peripherals.RADIO_CLK,
    // )
    // .unwrap();

    // TODO: Spawn some tasks
    let _ = spawner;

    let mut led = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());

    let mut mosfet_en = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());

    let analog_pin = peripherals.GPIO0;

    // type AdcCal = esp_hal::analog::adc::AdcCalBasic<esp_hal::peripherals::ADC1>;
    type AdcCal = esp_hal::analog::adc::AdcCalLine<esp_hal::peripherals::ADC1>;
    // type AdcCal = esp_hal::analog::adc::AdcCalCurve<esp_hal::peripherals::ADC1>;

    let mut adc1_config = AdcConfig::new();
    let mut adc1_pin = adc1_config.enable_pin_with_cal::<_, AdcCal>(analog_pin, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    loop {
        let adc_reading = nb::block!(adc1.read_oneshot(&mut adc1_pin)).unwrap();

        info!("Running pump...");
        mosfet_en.set_low(); // opto LED on → MOSFET closes
        Timer::after(Duration::from_secs(5)).await;
        mosfet_en.set_high(); // opto LED off → MOSFET opens
        info!("Stop!");

        info!("ADC reading: {} mV", adc_reading);
        // Dry: ~2800 mV
        // Water: ~1100 mV

        info!("Toggled LED...");
        led.toggle();
        Timer::after(Duration::from_secs(1)).await;
    }
}
