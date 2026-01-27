#include "esp_log.h"
#include "cJSON.h"
#include "command_helpers.h"
// TODO: Need to implement logic to track current, target RA and DEC and convert difference into steps
RA_diff_steps = 1000 //difference between current and target RA in steps of motor
DEC_diff_steps = 1000 //difference between current and target DEC in steps of motor

void handle_cmd_goto(cJSON *root)
{
    (void)root;

    ESP_LOGI("controller", "Goto received");
    controller_log_line("Goto received\n");

    // TODO: parse fields like "ra"/"dec"
    stepperRA.setMaxSpeed(max_speed);
    stepperRA.setAcceleration(accel);
    stepperRA.setSpeed(speed);
    stepperRA.moveTo(RA_diff_steps);

    stepperDEC.setMaxSpeed(max_speed);
    stepperDEC.setAcceleration(accel);
    stepperDEC.setSpeed(speed);
    stepperDEC.moveTo(DEC_diff_steps);

    RA_finished = false
    DEC_finished = false

    while (!DEC_finished and !RA_finished) {
        if (stepperRA.distanceToGo() == 0)
            RA_finished = true;
        else
            stepperRA.run();

        if (stepperDEC.distanceToGo() == 0)
            DEC_finished = true;
        else
            stepperDEC.run();
    }
}