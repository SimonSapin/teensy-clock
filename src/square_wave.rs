use core::ptr;
use teensy3;

const SQUARE_WAVE_PIN: u8 = 10;

#[derive(Copy, Clone)]
pub struct SquareWave;

static mut TICKED: bool = false;

impl SquareWave {
    pub fn init(self) {
        unsafe extern "C" fn tick() {
            ptr::write_volatile(&mut TICKED, true);
        }
        unsafe {
            teensy3::pinMode(SQUARE_WAVE_PIN, teensy3::INPUT_PULLUP as u8);
            teensy3::attachInterrupt(SQUARE_WAVE_PIN, Some(tick), teensy3::RISING as i32);
        }
    }

    pub fn ticked(self) -> bool {
        unsafe {
            let ticked = ptr::read_volatile(&TICKED);
            if ticked {
                ptr::write_volatile(&mut TICKED, false);
            }
            ticked
        }
    }
}
