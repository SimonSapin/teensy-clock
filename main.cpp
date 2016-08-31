#include "WProgram.h"
#define SERIAL_HEX HEX
#include "Adafruit_LEDBackpack.h"
#include "SPI.h"

uint32_t read_int(uint8_t separator = '\n') {
    uint8_t byte;
    uint32_t result = 0;
    for (;;) {
        byte = Serial.read();
        if (byte >= '0' && byte <= '9') {
            result *= 10;
            result += byte - '0';
        } else if (byte == separator) {
            return result;
        } else {
            return 0;
        }
    }
}

const int SPI_CHIP_SELECT = 6;

volatile bool ticked = true;

void tick() {
    ticked = true;
}

extern "C" int main(void) {
    pinMode(13, OUTPUT);

    Serial.begin(9600);

    Adafruit_7segment display;
    display.begin(0x70);
    display.setBrightness(3);

    pinMode(10, INPUT_PULLUP);
    attachInterrupt(digitalPinToInterrupt(10), tick, RISING);

    pinMode(SPI_CHIP_SELECT, OUTPUT);
    SPI.setMOSI(7);
    SPI.setMISO(8);
    SPI.setSCK(14);
    SPI.begin();
    SPI.setBitOrder(MSBFIRST);
    SPI.setDataMode(SPI_MODE1); // both mode 1 & 3 should work
    //set control register
    digitalWrite(SPI_CHIP_SELECT, LOW);
    SPI.transfer(0x8E);
    SPI.transfer(0x20);
    digitalWrite(SPI_CHIP_SELECT, HIGH);
    delay(10);

    uint32_t TimeDate [7]; //second,minute,hour,null,day,month,year
    while (1) {
        if (ticked) {
            ticked = false;

            for(int i=0; i<=6;i++){
                if(i==3)
                    i++;
                digitalWrite(SPI_CHIP_SELECT, LOW);
                SPI.transfer(i+0x00);
                unsigned int n = SPI.transfer(0x00);
                digitalWrite(SPI_CHIP_SELECT, HIGH);
                int a=n & B00001111;
                if(i==2){
                    int b=(n & B00110000)>>4; //24 hour mode
                    if(b==B00000010)
                        b=20;
                    else if(b==B00000001)
                        b=10;
                    TimeDate[i]=a+b;
                }
                else if(i==4){
                    int b=(n & B00110000)>>4;
                    TimeDate[i]=a+b*10;
                }
                else if(i==5){
                    int b=(n & B00010000)>>4;
                    TimeDate[i]=a+b*10;
                }
                else if(i==6){
                    int b=(n & B11110000)>>4;
                    TimeDate[i]=a+b*10;
                }
                else{
                    int b=(n & B01110000)>>4;
                    TimeDate[i]=a+b*10;
                }
            }

            display.print(TimeDate[0] + TimeDate[1] * 100 , DEC);
            display.drawColon(true);
            display.writeDisplay();

            digitalWriteFast(13, HIGH);
            delay(10);
            digitalWriteFast(13, LOW);
        }

        if (Serial.available()) {
            uint8_t byte;
            byte = Serial.read();
            if (byte == '@') {
                Serial.print("Got integer: ");
                Serial.println(read_int());
            } else if (byte == 'g') {  // get
                Serial.print("Current RTC datetime: ");
                Serial.print(TimeDate[6]);
                Serial.print("-");
                Serial.print(TimeDate[5]);
                Serial.print("-");
                Serial.print(TimeDate[4]);
                Serial.print(" ");

                Serial.print(TimeDate[2]);
                Serial.print(":");
                Serial.print(TimeDate[1]);
                Serial.print(":");
                Serial.println(TimeDate[0]);
            } else if (byte == 's') {  // set
                TimeDate[6] = read_int('-') % 100;
                TimeDate[5] = read_int('-');
                TimeDate[4] = read_int(' ');

                TimeDate[2] = read_int(':');
                TimeDate[1] = read_int(':');
                TimeDate[0] = read_int('\n');

                for(int i=0; i<=6;i++){
                    if(i==3)
                        i++;
                    int b= TimeDate[i]/10;
                    int a= TimeDate[i]-b*10;
                    if(i==2){
                        if (b==2)
                            b=B00000010;
                        else if (b==1)
                            b=B00000001;
                    }
                    TimeDate[i]= a+(b<<4);

                    digitalWrite(SPI_CHIP_SELECT, LOW);
                    SPI.transfer(i+0x80);
                    SPI.transfer(TimeDate[i]);
                    digitalWrite(SPI_CHIP_SELECT, HIGH);
                }
            }
        }
    }
}
