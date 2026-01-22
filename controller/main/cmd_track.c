#include "esp_log.h"
#include "cJSON.h"
#include "command_helpers.h"

// Creates an instance
// Pins entered in sequence IN1-IN3-IN2-IN4 for proper step sequence
AccelStepper myStepper(MotorInterfaceType, 8, 10, 9, 11);

void handle_cmd_start(cJSON *root)
{
    ESP_LOGI("controller", "Start received");
    controller_log_line("Start received\n");

    myStepper.runSpeed();
}

void handle_cmd_stop(cJSON *root) {
    //TODO handle stop
}