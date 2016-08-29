#include "WProgram.h"
#define SERIAL_HEX HEX
#include "Adafruit_LEDBackpack.h"

uint8_t read_byte() {
    while (!Serial.available()) {}
    return Serial.read();
}

uint32_t read_int() {
    uint8_t byte;
    uint32_t result = 0;
    for (;;) {
        byte = read_byte();
        if (byte >= '0' && byte <= '9') {
            result *= 10;
            result += byte - '0';
        } else if (byte == '\n') {
            return result;
        } else {
            return 0;
        }
    }
}

extern "C" int main(void) {
    pinMode(13, OUTPUT);
    digitalWriteFast(13, HIGH);
    delay(100);
    digitalWriteFast(13, LOW);

    Adafruit_7segment display;
    display.begin(0x70);
    display.setBrightness(3);
    Serial.begin(9600);
    int i = 1234;
    int j = 100;
    while (1) {
        if (j == 100000) {
            j = 0;
            display.print(i , DEC);
            display.drawColon(true);
            display.writeDisplay();
            i++;
            i %= 10000;
        }
        j++;

        if (Serial.available()) {
            uint8_t byte;
            byte = read_byte();
            if (byte == '@') {
                Serial.print("Got integer: ");
                Serial.println(read_int());
            }
        }
    }
}
