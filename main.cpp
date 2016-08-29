#include "WProgram.h"



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

    Serial.begin(9600);
    while (1) {
        uint8_t byte;
        byte = read_byte();
        if (byte == '@') {
            Serial.print("Got integer: ");
            Serial.println(read_int());
        }
    }
}
