#![no_std]
#![no_main]

extern crate ddos_ds as ds;

#[macro_use]
extern crate ddos_drivers as drivers;

use core::panic::PanicInfo;

#[panic_handler]
#[allow(clippy::empty_loop)]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("HELLO WORLD");

    #[allow(clippy::empty_loop)]
    loop {}
}
