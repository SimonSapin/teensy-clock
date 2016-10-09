use gregor::{DateTime, Utc, Month};
use teensy3::bindings as teensy3;
use teensy3::bindings::SPIClass;

const SPI_CHIP_SELECT_PIN: u8 = 6;
const SPI_MOSI_PIN: u8 = 7;
const SPI_MISO_PIN: u8 = 8;
const SPI_SCK_PIN: u8 = 14;

const CURRENT_DATETIME_ADDRESS: u8 = 0x00;
const CONTROL_REGISTER_ADDRESS: u8 = 0x0E;
const CONTROL_REGISTER_CONVERT_TEMPERATURE: u8 = 0x20;

#[derive(Copy, Clone)]
pub struct RTC;

impl RTC {
    pub fn init(self) {
        unsafe {
            teensy3::pinMode(SPI_CHIP_SELECT_PIN, teensy3::OUTPUT as u8);
            SPIClass::setMOSI(SPI_MOSI_PIN);
            SPIClass::setMISO(SPI_MISO_PIN);
            SPIClass::setSCK(SPI_SCK_PIN);
            SPIClass::begin();
            SPIClass::setBitOrder(teensy3::MSBFIRST as u8);
            SPIClass::setDataMode(teensy3::SPI_MODE1 as u8);
            self.spi_write(CONTROL_REGISTER_ADDRESS, &[
                CONTROL_REGISTER_CONVERT_TEMPERATURE,
            ]);
        }
    }

    pub fn get(self) -> DateTime<Utc> {
        let mut data = [0, 0, 0, 0, 0, 0, 0];
        self.spi_read(CURRENT_DATETIME_ADDRESS, &mut data);
        let second = bcd_decode(data[0]);
        let minute = bcd_decode(data[1]);
        let hour = bcd_decode(data[2]);
        // data[3] is the day of the week, but we don’t rely on the RTC for that.
        let day = bcd_decode(data[4]);
        let month = Month::from_number(bcd_decode(data[5])).unwrap();
        let year = 2000 + i32::from(bcd_decode(data[6]));
        DateTime::new(Utc, year, month, day, hour, minute, second)
    }

    pub fn set(self, datetime: &DateTime<Utc>) {
        self.spi_write(CURRENT_DATETIME_ADDRESS, &[
            bcd_encode(datetime.second()),
            bcd_encode(datetime.minute()),
            bcd_encode(datetime.hour()),
            0,  // Day of the week, unused
            bcd_encode(datetime.day()),
            bcd_encode(datetime.month().to_number()),
            bcd_encode((datetime.year() - 2000) as u8),
        ])
    }

    fn spi_read(self, address: u8, data: &mut [u8]) {
        unsafe {
            teensy3::digitalWrite(SPI_CHIP_SELECT_PIN, teensy3::LOW as u8);
            SPIClass::transfer(address);
            for byte in data {
                *byte = SPIClass::transfer(0);
            }
            teensy3::digitalWrite(SPI_CHIP_SELECT_PIN, teensy3::HIGH as u8);
        }
    }

    fn spi_write(self, address: u8, data: &[u8]) {
        unsafe {
            teensy3::digitalWrite(SPI_CHIP_SELECT_PIN, teensy3::LOW as u8);
            SPIClass::transfer(address + 0x80);
            for &byte in data {
                SPIClass::transfer(byte);
            }
            teensy3::digitalWrite(SPI_CHIP_SELECT_PIN, teensy3::HIGH as u8);
        }
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
