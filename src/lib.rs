#![feature(lang_items)]
#![no_std]

extern crate gregor;

#[allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]
mod bindings;

#[macro_use]
mod serial;

use bindings::{SPIClass, Wire};
use core::ptr;
use gregor::{DateTime, Utc, Month};
use serial::Serial;

const LED_PIN: u8 = 13;
const SQUARE_WAVE_PIN: u8 = 10;
const SPI_CHIP_SELECT_PIN: u8 = 6;
const SPI_MOSI_PIN: u8 = 7;
const SPI_MISO_PIN: u8 = 8;
const SPI_SCK_PIN: u8 = 14;
const DISPLAY_I2C_ADDRESS: u8 = 0x70;
const DISPLAY_BRIGHTNESS: u8 = 1;

#[no_mangle]
pub extern fn main() {
    unsafe {
        bindings::pinMode(LED_PIN, bindings::OUTPUT as u8);
        bindings::pinMode(SQUARE_WAVE_PIN, bindings::INPUT_PULLUP as u8);

        bindings::attachInterrupt(SQUARE_WAVE_PIN, Some(tick), bindings::RISING as i32);

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
            update_display()
        }

        if Serial.readable() {
            match Serial.read_byte() {
                b'g' => rtc_print(),
                b's' => rtc_sync(),
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

fn update_display() {
    let datetime = rtc_get();
    let first = datetime.minute();
    let second = datetime.second();
    display_write_digits([
        first / 10,
        first % 10,
        second / 10,
        second % 10,
    ], true);
}

fn read_int(delimiter: u8) -> u32 {
    Serial.try_read_int_until(delimiter).unwrap()
}

fn spi_read(address: u8, data: &mut [u8]) {
    unsafe {
        bindings::digitalWrite(SPI_CHIP_SELECT_PIN, bindings::LOW as u8);
        SPIClass::transfer(address);
        for byte in data {
            *byte = SPIClass::transfer(0);
        }
        bindings::digitalWrite(SPI_CHIP_SELECT_PIN, bindings::HIGH as u8);
    }
}

fn spi_write(address: u8, data: &[u8]) {
    unsafe {
        bindings::digitalWrite(SPI_CHIP_SELECT_PIN, bindings::LOW as u8);
        SPIClass::transfer(address);
        for &byte in data {
            SPIClass::transfer(byte);
        }
        bindings::digitalWrite(SPI_CHIP_SELECT_PIN, bindings::HIGH as u8);
    }
}

/// Binary-Coded Decimal
fn bcd_decode(n: u8) -> u8 {
    (n >> 4) * 10 + (n & 0xF)
}

fn bcd_encode(n: u8) -> u8 {
    assert!(n < 100);
    (n / 10) << 4 | (n % 10)
}

fn rtc_get() -> DateTime<Utc> {
    let mut data = [0, 0, 0, 0, 0, 0, 0];
    spi_read(0x00, &mut data);
    let second = bcd_decode(data[0]);
    let minute = bcd_decode(data[1]);
    let hour = bcd_decode(data[2]);
    // data[3] is the day of the week, but we don’t rely on the RTC for that.
    let day = bcd_decode(data[4]);
    let month = Month::from_number(bcd_decode(data[5])).unwrap();
    let year = 2000 + i32::from(bcd_decode(data[6]));
    DateTime::new(Utc, year, month, day, hour, minute, second)
}

fn rtc_set(datetime: &DateTime<Utc>) {
    spi_write(0x80, &[
        bcd_encode(datetime.second()),
        bcd_encode(datetime.minute()),
        bcd_encode(datetime.hour()),
        0,  // Day of the week, unused
        bcd_encode(datetime.day()),
        bcd_encode(datetime.month().to_number()),
        bcd_encode((datetime.year() - 2000) as u8),
    ])
}

fn rtc_print() {
    println!("Current RTC datetime: {:?}", rtc_get());
}

fn rtc_sync() {
    let year = read_int(b'-') as i32;
    let month = Month::from_number(read_int(b'-') as u8).unwrap();
    let day = read_int(b' ') as u8;
    let hour = read_int(b':') as u8;
    let minute = read_int(b':') as u8;
    let second = read_int(b'\n') as u8;
    rtc_set(&DateTime::new(Utc, year, month, day, hour, minute, second))
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
