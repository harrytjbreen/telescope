#include "esp_log.h"
#include "cJSON.h"
#include "command_helpers.h"

void handle_cmd_stop(cJSON *root)
{
    (void)root;

    ESP_LOGI("controller", "Stop received");
    controller_log_line("Stop received\n");

    // TODO: stop tracking logic
}