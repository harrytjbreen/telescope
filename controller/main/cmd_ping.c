#include "cJSON.h"
#include "protocol.h"
#include "command_helpers.h"
#include "esp_log.h"

void handle_cmd_ping(cJSON *root)
{
    (void)root;

    ESP_LOGI("controller", "Ping received");
    controller_log_line("Ping received\n");

    cJSON *pong = cJSON_CreateObject();
    cJSON_AddStringToObject(pong, "type", EVT_PONG);
    controller_send_json(pong);
    cJSON_Delete(pong);
}