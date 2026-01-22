#pragma once

const int dirPin = 2;
const int stepPin = 3;
const float sidereal_theta = 0.00007292115; // rad/s
const float Dm_Dt = 1.0; // motor diameter / DEC worm shaft diameter
const float steps_in_rotation = 2048.0;
const float Pi = 3.141593

void setup_motor(void);