#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"]
pub extern fn rust_eh_personality() {
}

#[lang = "panic_fmt"]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    loop {}
}

#[allow(non_camel_case_types)]
pub type c_int = i32;

extern {
    fn usb_serial_getchar() -> c_int;
}

#[no_mangle]
pub extern fn read_int(separator: u8) -> u32 {
    let mut result = 0;
    loop {
        let byte = unsafe {
            usb_serial_getchar()
        };
        if byte == -1 {
            return 0
        }
        let byte = byte as u8;
        if byte >= b'0' && byte <= b'9' {
            result *= 10;
            result += u32::from(byte - b'0');
        } else if byte == separator {
            return result
        } else {
            return 0
        }
    }
}

