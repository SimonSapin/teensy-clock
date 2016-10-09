#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate gregor;
extern crate teensy3;

#[macro_use]
mod serial;

mod ds3234;
mod ht16k33;
mod square_wave;

use ds3234::RTC;
use gregor::{DateTime, Utc, Month};
use ht16k33::{Display, Brightness};
use serial::Serial;
use square_wave::SquareWave;

#[no_mangle]
pub extern fn main() -> ! {
    RTC.init();
    Display.init(Brightness::_1);
    SquareWave.init();

    loop {
        if SquareWave.ticked() {
            let datetime = RTC.get();
            let first = datetime.minute();
            let second = datetime.second();
            Display.write_digits([
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

fn read_int(delimiter: u8) -> u32 {
    Serial.try_read_int_until(delimiter).unwrap()
}

#[lang = "panic_fmt"]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("Panic at {}:{}, {}", file, line, msg);
    loop {}
}

#[lang = "eh_personality"]
pub extern fn rust_eh_personality() {}
