#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate gregor;
#[macro_use] extern crate teensy3;

mod ds3234;
mod ht16k33;
mod square_wave;

use ds3234::RTC;
use gregor::{DateTime, Utc, Month};
use ht16k33::{Display, Brightness};
use square_wave::SquareWave;
use teensy3::bindings;
use teensy3::serial::Serial;

#[lang = "panic_fmt"]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("Panic at {}:{}, {}", file, line, msg);
    loop {}
}

#[no_mangle]
pub extern fn main() -> ! {
    unsafe {
        bindings::pinMode(LED, bindings::OUTPUT as u8);
        bindings::delay(100);
    }
    RTC.init();
    Display.init(Brightness::_0);
    SquareWave.init();
    const LED: u8 = 13;
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
//            unsafe {
//                bindings::digitalWrite(LED, bindings::HIGH as u8);
//                bindings::delay(20);
//                bindings::digitalWrite(LED, bindings::LOW as u8);
//            }
        }

        if Serial.readable() {
            match Serial.try_read_byte() {
                Ok(b'g') => {
                    println!("Current RTC datetime: {:?}", RTC.get());
                }
                Ok(b's') => {
                    match read_datetime() {
                        Ok(datetime) => {
                            println!("RTC set to {:?}", datetime);
                            RTC.set(&datetime)
                        }
                        Err(err) => {
                            println!("Error reading datetime from USB serial: {}", err);
                        }
                    }
                }
                // Ignore unexpected bytes or reading errors
                _ => {}
            }
        }
    }
}

fn read_datetime() -> Result<DateTime<Utc>, &'static str> {
    let year = try!(read_int(&[b'-'])) as i32;
    let month = Month::from_number(try!(read_int(&[b'-'])) as u8).unwrap();
    let day = try!(read_int(&[b' '])) as u8;
    let hour = try!(read_int(&[b':'])) as u8;
    let minute = try!(read_int(&[b':'])) as u8;
    let second = try!(read_int(&[b'\n', b'\r'])) as u8;
    Ok(DateTime::new(Utc, year, month, day, hour, minute, second))
}

fn read_int(delimiters: &[u8]) -> Result<u32, &'static str> {
    let mut result = 0;
    loop {
        let byte = try!(Serial.try_read_byte());
        if delimiters.contains(&byte) {
            return Ok(result)
        }
        let digit = try!((byte as char).to_digit(10).ok_or("expected a decimal digit"));
        result *= 10;
        result += digit;
    }
}
