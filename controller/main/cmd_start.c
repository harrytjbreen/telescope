#include "esp_log.h"
#include "cJSON.h"
#include "command_helpers.h"

void handle_cmd_start(cJSON *root)
{
    (void)root;

    ESP_LOGI("controller", "Start received");
    controller_log_line("Start received\n");

    // TODO: start tracking logic
}