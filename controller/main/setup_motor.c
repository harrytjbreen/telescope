#include "setup_motor.h"
#include <AccelStepper.h> // Allegedy works with ESP mc

#define motorInterfaceType 1 // Always this for DRV8825
AccelStepper stepperRA(motorInterfaceType, stepPinRA, dirPinRA);
AccelStepper stepperDEC(motorInterfaceType, stepPinDEC, dirPinDEC);


void setup_motor() {
  Serial.begin(9600);
  Serial.println("Started");
}