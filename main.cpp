#include "WProgram.h"
#define SERIAL_HEX HEX
#include "Adafruit_LEDBackpack.h"
#include "SPI.h"

extern "C" {
    uint32_t read_int(uint8_t separator);
    void rust_init();
    void rtc_print();
    void rtc_sync();
    void update_display();
}

const int SPI_CHIP_SELECT = 6;

volatile bool ticked = true;

void tick() {
    ticked = true;
}

extern "C" int main(void) {
    rust_init();

    attachInterrupt(digitalPinToInterrupt(10), tick, RISING);

    uint32_t TimeDate [7]; //second,minute,hour,null,day,month,year
    while (1) {
        if (ticked) {
            ticked = false;

            update_display();
        }

        if (Serial.available()) {
            uint8_t byte;
            byte = Serial.read();
            if (byte == '@') {
                Serial.print("Got integer: ");
                Serial.println(read_int('\n'));
            } else if (byte == 'g') {
                rtc_print();
            } else if (byte == 's') {
                rtc_sync();
            }
        }
    }
}
