#include "setup_motor.h"

void setup_motor() {
  Serial.begin(9600);
  pinMode(stepPin, OUTPUT);
  pinMode(dirPin, OUTPUT);
  Serial.println("Started");
}