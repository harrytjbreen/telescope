#include "esp_log.h"
#include "cJSON.h"
#include "command_helpers.h"

void handle_cmd_goto(cJSON *root)
{
    (void)root;

    ESP_LOGI("controller", "Goto received");
    controller_log_line("Goto received\n");

    // TODO: parse fields like "ra"/"dec"
}