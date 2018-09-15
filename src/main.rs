// in main.rs

#![no_std]
#![feature(panic_implementation)] 
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic 
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop{}
}
