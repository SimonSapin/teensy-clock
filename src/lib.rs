#![feature(lang_items)]
#![no_std]

extern crate gregor;

#[allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]
#[path = "bindings.rs"]
mod teensy3;

#[macro_use]
mod serial;

mod ds3234;

use core::ptr;
use ds3234::RTC;
use gregor::{DateTime, Utc, Month};
use serial::Serial;
use teensy3::Wire;

const SQUARE_WAVE_PIN: u8 = 10;
const DISPLAY_I2C_ADDRESS: u8 = 0x70;
const DISPLAY_BRIGHTNESS: u8 = 1;

#[no_mangle]
pub extern fn main() {
    unsafe {
        teensy3::pinMode(SQUARE_WAVE_PIN, teensy3::INPUT_PULLUP as u8);
        teensy3::attachInterrupt(SQUARE_WAVE_PIN, Some(tick), teensy3::RISING as i32);
    }

    RTC.init();

    const HT16K33_OSCILLATOR_ON_COMMAND: u8 = 0x21;
    const HT16K33_BLINK_COMMAND: u8 = 0x80;
    const HT16K33_BLINK_DISPLAY_ON: u8 = 0x01;
    const HT16K33_BLINK_OFF: u8 = 0x00;
    const HT16K33_BRIGHTNESS_COMMAND: u8 = 0xE0;
    i2c_write(&[HT16K33_OSCILLATOR_ON_COMMAND]);
    i2c_write(&[HT16K33_BLINK_COMMAND | HT16K33_BLINK_DISPLAY_ON | HT16K33_BLINK_OFF]);
    i2c_write(&[HT16K33_BRIGHTNESS_COMMAND | check_ht16k33_brightness(DISPLAY_BRIGHTNESS)]);

    loop {
        if ticked() {
            let datetime = RTC.get();
            let first = datetime.minute();
            let second = datetime.second();
            display_write_digits([
                first / 10,
                first % 10,
                second / 10,
                second % 10,
            ], true);
        }

        if Serial.readable() {
            match Serial.read_byte() {
                b'g' => {
                    println!("Current RTC datetime: {:?}", RTC.get());
                }
                b's' => {
                    let year = read_int(b'-') as i32;
                    let month = Month::from_number(read_int(b'-') as u8).unwrap();
                    let day = read_int(b' ') as u8;
                    let hour = read_int(b':') as u8;
                    let minute = read_int(b':') as u8;
                    let second = read_int(b'\n') as u8;
                    RTC.set(&DateTime::new(Utc, year, month, day, hour, minute, second))
                }
                _ => {}
            }
        }
    }
}

static mut TICKED: bool = false;

fn ticked() -> bool {
    unsafe {
        let ticked = ptr::read_volatile(&TICKED);
        if ticked {
            ptr::write_volatile(&mut TICKED, false);
        }
        ticked
    }
}

unsafe extern "C" fn tick() {
    ptr::write_volatile(&mut TICKED, true);
}

fn check_ht16k33_brightness(brightness: u8) -> u8 {
    assert!(brightness <= 0x0F);
    brightness
}

fn i2c_write(data: &[u8]) {
    unsafe {
        Wire.begin();
        Wire.beginTransmission(DISPLAY_I2C_ADDRESS);
        for &byte in data {
            Wire.send(byte);
        }
        // FIXME: what does this return value mean? Should we handle errors?
        let _result: u8 = Wire.endTransmission();
    }
}

const ADAFRUIT_7_SEGMENTS_COLON: u8 = 0x02;

fn display_write_segments(segments: [u8; 4], colon: bool) {
    i2c_write(&[
        0x00,  // address
        segments[0],
        0x00,  // Unused
        segments[1],
        0x00,  // Unused
        if colon { ADAFRUIT_7_SEGMENTS_COLON } else { 0 },
        0x00,  // Unused
        segments[2],
        0x00,  // Unused
        segments[3],
    ]);
}

const ADAFRUIT_7_SEGMENTS_DIGITS: [u8; 10] = [
    0x3F,  // 0
    0x06,  // 1
    0x5B,  // 2
    0x4F,  // 3
    0x66,  // 4
    0x6D,  // 5
    0x7D,  // 6
    0x07,  // 7
    0x7F,  // 8
    0x6F,  // 9
];

fn display_write_digits(digits: [u8; 4], colon: bool) {
    display_write_segments([
        ADAFRUIT_7_SEGMENTS_DIGITS[digits[0] as usize],
        ADAFRUIT_7_SEGMENTS_DIGITS[digits[1] as usize],
        ADAFRUIT_7_SEGMENTS_DIGITS[digits[2] as usize],
        ADAFRUIT_7_SEGMENTS_DIGITS[digits[3] as usize],
    ], colon);
}

fn read_int(delimiter: u8) -> u32 {
    Serial.try_read_int_until(delimiter).unwrap()
}


mod std {
    pub use core::*;
    pub mod os {
        #[allow(non_camel_case_types)]
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
