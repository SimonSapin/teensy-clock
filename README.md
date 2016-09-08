I’ve previously played with
and [written about](https://github.com/SimonSapin/rust-on-bbc-microbit#readme)
Rust on [BBC micro:bit](http://microbit.co.uk/),
but kind gave up on it because it’s somewhat difficult
to make good electrical connection with the small pins on the edge connector
and I was getting inconsistent brightness when driving LEDs
directly from the microcontroller’s digital I/O pins
and don’t know why.

So I’ve switched to [Teensy](https://www.pjrc.com/teensy/teensy31.html)
and got started with the
[Bare Metal Rust on the Teensy 3.1](http://disconnected.systems/bare-metal-rust-on-the-teensy-3.1/)
blog post.
While not having any C/C++ code (only Rust and a linker script) seems attractive,
it leaves you re-inverting the entire hardware abstraction layer,
which is not so fun when doing stuff more involved than blinking a LED.

So I want to use [Teensyduino](https://www.pjrc.com/teensy/teensyduino.html),
the runtime software environment based on Arduino that everyone else uses for Teensy.
But I don’t want to use the Arduino IDE.
It turns out that the relevant code is available in
the [PaulStoffregen/cores](https://github.com/PaulStoffregen/cores/) repository
with a example Makefile that uses the `arm-none-eabi-gcc` toolchain.

So far I’ve managed to:

* Add a cross-compiled Rust static library into this mix
* Use [Servo’s rust-bindgen](https://github.com/servo/rust-bindgen) (which has some C++ support)
  to automatically generate Rust bindings for all of Teensyduino.
* Have Rust code run on the Teensy and use these bindings for:
  * USB serial with a Linux laptop
  * i2c / TwoWire with an LED driver chip
  * SPI with an RTC chip
  * Setting up an interrupt on digital I/O input

This is what’s in this repository.

I’ve then tried to flip the build system around so that Rust makes an executable
and the C/C++ code is a couple of static libraries built with the `gcc` crate (yay, no Makefile!)
…but that doesn’t seem to boot and I don’t know why. (Yet, hopefully.)
This can be found in [the `rustbuild` branch](
https://github.com/SimonSapin/teensy-clock/compare/rustbuild).
Any help with this is appreciated!

The prototyping setup looks like this:

<img src=pictures/proto.jpg height=500>

And this is close to what I want to hang on my wall eventually.
A clock that updates for summer time (a.k.a. daylight saving time) automatically.

<img src=pictures/build1.jpg height=300>
<img src=pictures/build2.jpg height=300>
