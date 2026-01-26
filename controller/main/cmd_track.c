#include "esp_log.h"
#include "cJSON.h"
#include "command_helpers.h"

float sidereal_omega_steps = sidereal_theta * steps_in_rotation / (2 * Pi)

void handle_cmd_start(cJSON *root)
{
    ESP_LOGI("controller", "Start received");
    controller_log_line("Start received\n");

    // Set motor direction clockwise, LOW for anti-clockwise
    stepperRA.setMaxSpeed(sidereal_omega_steps);
    stepperRA.setSpeed(sidereal_omega_steps);

    while (1) {
        stepperRA.runSpeed();
    }
}

void handle_cmd_stop(cJSON *root) {
    //TODO handle stop
}