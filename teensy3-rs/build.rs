use std::process::Command;

fn main() {
    run("make", &["-C", "../teensy3", "NO_ARDUINO=1", "libteensy3.a"]);
    println!("cargo:rustc-link-lib=static=teensy3");
    println!("cargo:rustc-link-search={}/../teensy3", env!("CARGO_MANIFEST_DIR"));
    println!("cargo:rerun-if-changed={}/../teensy3", env!("CARGO_MANIFEST_DIR"));
}

fn run(exe: &str, args: &[&str]) {
    assert!(Command::new(exe).args(args).spawn().unwrap().wait().unwrap().success());
}
