#include <stdarg.h>
#include <string.h>
#include <stdio.h>

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "driver/uart.h"
#include "esp_log.h"
#include "cJSON.h"
#include "protocol.h"
#include "controller_config.h"
#include "command_helpers.h"
#include "command_handlers.h"
#include "setup_motor.h"

static const char *TAG = "controller";

#define LOG_BUF_SIZE 2048
static char log_buf[LOG_BUF_SIZE];
static size_t log_pos = 0;

static void dispatch_command(const char *type, cJSON *root)
{
    if (!type) {
        return;
    }

    if (strcmp(type, CMD_PING) == 0) {
        handle_cmd_ping(root);
        return;
    }

    if (strcmp(type, CMD_START) == 0) {
        handle_cmd_start(root);
        return;
    }

    if (strcmp(type, CMD_STOP) == 0) {
        handle_cmd_stop(root);
        return;
    }

    if (strcmp(type, CMD_GOTO) == 0) {
        handle_cmd_goto(root);
        return;
    }
}

void app_main(void)
{
    uart_config_t uart_config = {
        .baud_rate = 115200,
        .data_bits = UART_DATA_8_BITS,
        .parity    = UART_PARITY_DISABLE,
        .stop_bits = UART_STOP_BITS_1,
        .flow_ctrl = UART_HW_FLOWCTRL_DISABLE,
    };

    uart_driver_install(UART_NUM, 1024, 0, 0, NULL, 0);
    uart_param_config(UART_NUM, &uart_config);

    ESP_LOGI(TAG, "Controller ready");
    controller_log_line("Controller ready\n");

    static char line_buf[512];
    static size_t line_len = 0;

    setup_motor();

    while (1) {
        uint8_t rx_buf[128];

        int len = uart_read_bytes(
            UART_NUM,
            rx_buf,
            sizeof(rx_buf),
            pdMS_TO_TICKS(1000)
        );

        if (len <= 0) {
            continue;
        }

        for (int i = 0; i < len; i++) {
            char c = rx_buf[i];

            if (c == '\n') {
                line_buf[line_len] = '\0';

                ESP_LOGI(TAG, "RX: %s", line_buf);

                cJSON *root = cJSON_Parse(line_buf);
                if (!root) {
                    ESP_LOGW(TAG, "Invalid JSON");
                } else {
                    cJSON *type = cJSON_GetObjectItem(root, "type");
                    if (cJSON_IsString(type)) {
                        dispatch_command(type->valuestring, root);
                    } else {
                        ESP_LOGW(TAG, "Missing type field");
                    }
                    cJSON_Delete(root);
                }

                line_len = 0;
            } else if (line_len < sizeof(line_buf) - 1) {
                line_buf[line_len++] = c;
            }
        }
    }
}