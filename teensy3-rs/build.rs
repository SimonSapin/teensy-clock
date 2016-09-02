extern crate gcc;
extern crate glob;

const MCU: &'static str = "mk20dx256";

fn main() {
    common(&mut gcc::Config::new(), "../teensy3/*.c")
        .compiler("arm-none-eabi-gcc")
        .compile("libteensyduino_c.a");

    common(&mut gcc::Config::new(), "../teensy3/*.cpp")
        .flag("-felide-constructors")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-fkeep-inline-functions")
        .flag("-std=gnu++0x")
        .cpp(true)
        .compiler("arm-none-eabi-g++")
        .compile("libteensyduino_cpp.a");

    println!("cargo:rerun-if-changed=../teensy3")
}

fn common<'a>(lib: &'a mut gcc::Config, pattern: &str) -> &'a mut gcc::Config {
    for file in glob::glob(pattern).unwrap() {
        lib.file(file.unwrap());
    }
    lib
        .include("../teensy3")
        .define("F_CPU", Some("48000000"))
        .define("USB_SERIAL", None)
        .define("LAYOUT_US_ENGLISH", None)
        .define("CORE_TEENSY", None)
        .define(&format!("__{}__", MCU.to_uppercase()), None)
        .define("ARDUINO", Some("10600"))
        .define("TEENSYDUINO", Some("121"))
        .flag("-Wall")
        .flag("-Os")
        .flag("-mcpu=cortex-m4")
        .flag("-mthumb")
        .archiver("arm-none-eabi-ar")
}
