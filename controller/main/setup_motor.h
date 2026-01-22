#pragma once

// Define step constant (8 = half-step, smoother control)
#define MotorInterfaceType 8

#include <AccelStepper.h>
const float sidereal_theta = 0.00007292115; // rad/s
const float Dm_Dt = 1.0; // motor diameter / DEC worm shaft diameter
const float steps_in_rotation = 2*2048.0;

void setup_motor(void);