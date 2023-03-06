#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;

use panic_halt as _;

use cortex_m_rt;
use nrf52840_hal::clocks::Clocks;
use nrf52840_hal::usbd::{UsbPeripheral, Usbd};
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[cortex_m_rt::entry]
fn main() -> ! {
    hprintln!("Test").unwrap();
    let periph = nrf52840_hal::pac::Peripherals::take().unwrap();
    let clocks = Clocks::new(periph.CLOCK);
    let clocks = clocks.enable_ext_hfosc();

    let usb_bus = UsbBusAllocator::new(Usbd::new(UsbPeripheral::new(periph.USBD, &clocks)));
    let mut serial = SerialPort::new(&usb_bus);

    // let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(4966, 4177))
        .manufacturer("SEGGER")
        .product("J-Link")
        .serial_number("001050281902")
        .device_class(USB_CLASS_CDC)
        // .max_packet_size_0(64) // (makes control transfers 8x faster)
        .build();
    let mut b = false;
    loop {
        hprintln!("loop").unwrap();
        serial.write(b"test").unwrap();

        // if !usb_dev.poll(&mut [&mut serial]) {
        //     if !b {
        //         b = true;
        //     }
        //     continue;
        // }
        // hprintln!("loop").unwrap();
        
        // let mut buf = [0u8; 32];
        
        // hprintln!("reading").unwrap();
        // match serial.read(&mut buf) {
        //     Ok(_) => {
        //         hprintln!("serial.read ok").unwrap();

        //         serial.write(b"test").unwrap();
        //     }
        //     _ => {
        //         hprintln!("serial.read NOK").unwrap();
        //     }
        // }
    }
}
