#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use nrf52840_hal as _; // memory layout
use defmt_rtt as _; // global logger


#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    // defmt::info!("processor temp is °C");
    

    loop {}
}
