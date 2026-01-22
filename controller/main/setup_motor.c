#include "setup_motor.h"

void setup_motor() {
  Serial.begin(9600);
  float motor_theta = sidereal_theta / Dm_Dt;
  float motor_speed_steps = sidereal_theta * steps_in_rotation / (2 * PI);
  // set the maximum speed, acceleration factor,
  // initial speed and the target position
  myStepper.setMaxSpeed(motor_speed_steps);
  myStepper.setSpeed(motor_speed_steps);
  Serial.println("Started");
}