//! Using Adafruit’s  0.56" 7-Segment LED Backpack
//! https://learn.adafruit.com/adafruit-led-backpack/0-dot-56-seven-segment-backpack

use teensy3::Wire;

#[derive(Copy, Clone)]
pub struct Display;

const DISPLAY_I2C_ADDRESS: u8 = 0x70;

const OSCILLATOR_ON_COMMAND: u8 = 0x21;
const BLINK_COMMAND: u8 = 0x80;
const BLINK_DISPLAY_ON: u8 = 0x01;
const BLINK_OFF: u8 = 0x00;
const BRIGHTNESS_COMMAND: u8 = 0xE0;

const ADAFRUIT_7_SEGMENTS_COLON: u8 = 0x02;
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

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Brightness {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
    _9 = 9,
    _10 = 10,
    _11 = 11,
    _12 = 12,
    _13 = 13,
    _14 = 14,
    _15 = 15,
}

impl Display {
    pub fn init(self, brightness: Brightness) {
        i2c_write(&[OSCILLATOR_ON_COMMAND]);
        i2c_write(&[BLINK_COMMAND | BLINK_DISPLAY_ON | BLINK_OFF]);
        i2c_write(&[BRIGHTNESS_COMMAND | brightness as u8]);
    }

    pub fn write_segments(self, segments: [u8; 4], colon: bool) {
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

    pub fn write_digits(self, digits: [u8; 4], colon: bool) {
        self.write_segments([
            ADAFRUIT_7_SEGMENTS_DIGITS[digits[0] as usize],
            ADAFRUIT_7_SEGMENTS_DIGITS[digits[1] as usize],
            ADAFRUIT_7_SEGMENTS_DIGITS[digits[2] as usize],
            ADAFRUIT_7_SEGMENTS_DIGITS[digits[3] as usize],
        ], colon);
    }
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
