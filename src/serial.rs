use core::fmt;

extern {
    fn usb_serial_getchar() -> i32;
    fn usb_serial_write(ptr: *const u8, len: u32);
}

#[derive(Copy, Clone)]
pub struct Serial;

impl Serial {
    pub fn read_byte(self) -> u8 {
        self.try_read_byte().unwrap()
    }

    pub fn try_read_byte(self) -> Result<u8, &'static str> {
        match unsafe { usb_serial_getchar() } {
            -1 => Err("usb_serial_getchar returned -1"),
            byte => Ok(byte as u8)
        }
    }

    pub fn try_read_int_until(self, delimiter: u8) -> Result<u32, &'static str> {
        let mut result = 0;
        loop {
            let byte = try!(self.try_read_byte());
            if byte == delimiter {
                return Ok(result)
            }
            let digit = try!((byte as char).to_digit(10).ok_or("expected a decimal digit"));
            result *= 10;
            result += digit;
        }
    }

    pub fn write_bytes(self, bytes: &[u8]) {
        unsafe {
            usb_serial_write(bytes.as_ptr(), bytes.len() as u32)
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        write!($crate::serial::Serial, $($arg)*).unwrap()
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        writeln!($crate::serial::Serial, $($arg)*).unwrap()
    }
}
