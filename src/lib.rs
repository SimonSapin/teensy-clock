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
