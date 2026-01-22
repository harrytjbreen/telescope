#include "esp_log.h"
#include "cJSON.h"
#include "command_helpers.h"

float sidereal_omega_steps = sidereal_theta * steps_in_rotation / (2 * Pi)
float float microsecond_steps_gap = 1e6/(2*sidereal_omega_steps) // halved as gap before and after step

void handle_cmd_start(cJSON *root)
{
    ESP_LOGI("controller", "Start received");
    controller_log_line("Start received\n");

    // Set motor direction clockwise, LOW for anti-clockwise
    digitalWrite(dirPin, HIGH);

    while (1) {
        digitalWrite(stepPin, HIGH);
        delayMicroseconds(microsecond_steps_gap);
        digitalWrite(stepPin, LOW);
        delayMicroseconds(microsecond_steps_gap);
    }
}

void handle_cmd_stop(cJSON *root) {
    //TODO handle stop
}