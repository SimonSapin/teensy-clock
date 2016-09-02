#![feature(lang_items)]
#![no_std]

#[allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]
mod bindings;

#[macro_use]
mod serial;

use bindings::SPIClass;
use serial::Serial;

const LED_PIN: u8 = 13;
const SQUARE_WAVE_PIN: u8 = 10;
const SPI_CHIP_SELECT_PIN: u8 = 6;
const SPI_MOSI_PIN: u8 = 7;
const SPI_MISO_PIN: u8 = 8;
const SPI_SCK_PIN: u8 = 14;

#[no_mangle]
pub extern fn rust_init() {
    unsafe {
        bindings::pinMode(LED_PIN, bindings::OUTPUT as u8);
        bindings::pinMode(SQUARE_WAVE_PIN, bindings::INPUT_PULLUP as u8);

        bindings::pinMode(SPI_CHIP_SELECT_PIN, bindings::OUTPUT as u8);
        SPIClass::setMOSI(SPI_MOSI_PIN);
        SPIClass::setMISO(SPI_MISO_PIN);
        SPIClass::setSCK(SPI_SCK_PIN);
        SPIClass::begin();
        SPIClass::setBitOrder(bindings::MSBFIRST as u8);
        SPIClass::setDataMode(bindings::SPI_MODE1 as u8);
        bindings::digitalWrite(SPI_CHIP_SELECT_PIN, bindings::LOW as u8);
        SPIClass::transfer(0x8E);
        SPIClass::transfer(0x20);
        bindings::digitalWrite(SPI_CHIP_SELECT_PIN, bindings::HIGH as u8);
    }
}

#[no_mangle]
pub extern fn read_int(delimiter: u8) -> u32 {
    Serial.try_read_int_until(delimiter).unwrap()
}


mod std {
    pub use core::*;
    pub mod os {
        #[allow(non_camel_case_types, dead_code)]
        pub mod raw {
            pub enum c_void {}
            pub type c_uchar = u8;
            pub type c_short = i16;
            pub type c_ushort = u16;
            pub type c_int = i32;
            pub type c_uint = u32;
            pub type c_long = i32;
            pub type c_ulong = u32;
            pub type c_longlong = i64;
            pub type c_ulonglong = u64;
        }
    }
}

#[lang = "panic_fmt"]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("Panic at {}:{}, {}", file, line, msg);
    loop {}
}

#[lang = "eh_personality"]
pub extern fn rust_eh_personality() {}
