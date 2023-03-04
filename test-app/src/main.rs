#![no_main]
#![no_std]

use core::borrow::Borrow;

use nrf52840_hal as _; // memory layout
use panic_halt as _;

use nb::block;
use nrf52840_dk_bsp::{
    hal::{
        prelude::*,
        timer::{self, Timer},
        Temp,
    },
    Board,
};

use cortex_m_semihosting::hprintln;
// use nrf52840_hal::pac::Peripherals;
// use nrf52840_hal::temp::Temp;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut nrf52 = Board::take().unwrap();

    let mut timer = Timer::new(nrf52.TIMER0);

    let mut temp_sensor = Temp::new(nrf52.TEMP);

    loop {
        nrf52.leds.led_2.enable();
        delay(&mut timer, 250_000); // 250ms
        nrf52.leds.led_2.disable();
        delay(&mut timer, 1_000_000); // 1s

        let temp_c: i32 = temp_sensor.measure().to_num();
        hprintln!("Temp {}", temp_c).unwrap();
    }
}

fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
    T: timer::Instance,
{
    timer.start(cycles);
    let _ = block!(timer.wait());
}
