# Hello Teensy

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

I’ve managed to:

* Add a cross-compiled Rust static library into this mix
* Use [Servo’s rust-bindgen](https://github.com/servo/rust-bindgen) (which has some C++ support)
  to automatically generate Rust bindings for all of Teensyduino.
* Have Rust code run on the Teensy and use these bindings for:
  * USB serial with a Linux laptop
  * i2c / TwoWire with an LED driver chip
  * SPI with an RTC chip
  * Setting up an interrupt on digital I/O input

The prototyping setup looks like this:

<img src=pictures/proto.jpg height=500>


# teensy3-rs

I went to [RustFest EU](http://www.rustfest.eu/) 2016 and attended the Embedded workshop there.
There I met several people doing similar things.
James Munns in particular was interested in getting Rust running on Teensy 3.x hardware,
and making it easier for other people to do so.
Together we flipped the build system that I had on its head so that it is mainly driven by Cargo,
with a build script compiling the C and C++ code from Teensyduino.
With a bit more polish,
this became the [teensy3-rs](https://github.com/jamesmunns/teensy3-rs/) project.

Depending on the `teensy3` crate from crates.io gets you:

* C and C++ code from Teensyduino for start-up and hardware abstraction.
* Generated low-level Rust bindings for its entire API, generated.
  (Every function is `unsafe`, constants may require type conversions.)
* Higher-level API for some of the functionality, such as `println!` going over USB serial.

Projects using this still require some boilerplate that can not be in a Cargo dependency.
This can be copied from [teensy3-rs-demo](https://github.com/jamesmunns/teensy3-rs-demo).


# Gregor

Several years before, I had built another clock with a four-digit LED display,
an RTC chip, and an AVR microcontroller.

<img src=pictures/avr_clock.jpg height=500>

The C++ code for it knows about dates and days of the week,
but I never ended up writing the code
to switch to and from summer time (a.k.a. daylight saving time) automatically.
Every six months or so I would tell myself I really ought to do it some day.
Then I’d look at my crappy C++ code, give up, change the UTC offset constant,
recompile, and take my clock off the wall to reflash its firmware.

This is my main motivation for all this.
Rather than fix up an old project, it’s much more fun to make a new one in Rust!

And so I wrote [Gregor](https://github.com/SimonSapin/gregor),
a Rust library for dealing with timestamps, calendaring, time zones, and summer time / DST.

Here it is in action. The GIF starts with the clock set to 2016-10-30 00:59:56 UTC
to see the switch to winter time:

<img src=pictures/winter_time.gif>


# Myopia is color-sensitive?

My old AVR clock has a red LED display.
For this new clock I bought the blue one because it looks cool
(pictured above in the prototyping setup),
but I hit an unexpected issue: it is not compatible… with my eyes.

I have myopia (nearsightedness):
without glasses I can’t focus on far-away things and see them blurry.
I usually don’t wear my glasses indoor because my myopia is mild enough
that I don’t need them at all for anything closer than 1.5 meters or so.

The red display is about 2 centimeters tall, big enough that I can read it
from several meters away without glasses.
But somehow, that doesn’t work with the otherwise identical blue display!
To me it becomes blurry much quicker as distances increase.

I didn’t know that myopia was affected by color but experimentally it is, at least for me.
I didn’t manage to find information about this directly,
other than that color is used in [a test to tell myopia form hyperopia](
http://www.essilor.com/en/EyeHealth/LensesForYourVision/TestyourEyes/Pages/Eyetestformyopiaandhyperopia.aspx).

So, red display it is.


# Brightness, contrast, and gelatin

I want the display brightness to be low
so that it doesn’t illuminate the room at night with the lights off.
But I also want to read it during the day, so the contrast between segments that are on and off
needs to be high enough.

7-segment displays as they are sold have a dark background, but unlit segments look white.
This makes for poor readability at low brightness with ambient light.
The trick is to use a colored light filter, so that these segments are less visible.

A thing relatively easy to buy online and use is “gelatin”:
flexible transparent colored film normally used in theaters or concert halls on light projectors.
With some cutting, folding, and adhesive tape it works out very nicely.

<img src=pictures/gelatin_00.jpg height=200>
<img src=pictures/gelatin_01.jpg height=200>
<img src=pictures/gelatin_02.jpg height=200>
<img src=pictures/gelatin_03.jpg height=200>
<img src=pictures/gelatin_04.jpg height=200>
<img src=pictures/gelatin_05.jpg height=200>
<img src=pictures/gelatin_06.jpg height=200>
<img src=pictures/gelatin_07.jpg height=200>
<img src=pictures/gelatin_08.jpg height=200>


# Final build

Here are the parts I ended up using:

* A [Teensy](https://www.pjrc.com/teensy/index.html) 3.2 microcontroller
* A USB power supply and micro-USB cable from an old phone
* A [SparkFun DeadOn RTC](https://shop.pimoroni.com/products/sparkfun-deadon-rtc-ds3234-breakout)
  (breakout board for DS3234 chip).
* A CR1220 coin-cell battery to keep the RTC (“real-time clock”) running when USB power is off.
* An [Adafruit 0.56" 7-Segment LED Backpack](
  https://learn.adafruit.com/adafruit-led-backpack/0-dot-56-seven-segment-backpack).
  It deals with multiplexing and providing constant current (not voltage) to…
* A red (not blue) [4-digit 7-segment LED display](
  https://shop.pimoroni.com/products/adafruit-0-56-4-digit-7-segment-display-w-i2c-backpack)
  that came with its controller “backpack”.
* A couple 4.7 kΩ resistors because [for some reason](https://www.pjrc.com/teensy/td_libs_Wire.html)
  the Teensy’s internal pull-up resistors don’t work for I²C.
* A bit of foam for insulation, cut from the packaging of a computer GPU.
  (GPU not required for this clock.)
* [“Jumper lead” wires](https://shop.pimoroni.com/collections/prototyping/products/jumper-lead-selection).
  They’re designed for use with a breadboard, but are still very convenient for this:
  the come insulated, in various lengths, with pre-stripped ends, and with solid (not stranded)
  core so that they keep their shape after bending.
  Once soldered they provide fairly good mechanical connection,
  which means this project doesn’t need a “main” board since the whole thing is rather light.

<img src=pictures/build1.jpg height=300>
<img src=pictures/build2.jpg height=300
     title="This photo is from before I switched the blue display to a red one.">

# Deployment to production

Tonight, a week early for the switch to winter time in October 2016,
I deployed this project to production.
By which I mean I finished soldering it and hung it on my bedroom wall.

<img src=pictures/deployed.jpg height=500>
