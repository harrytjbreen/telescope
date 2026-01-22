#include <stdarg.h>
#include <string.h>
#include <stdio.h>

#include "driver/uart.h"
#include "cJSON.h"
#include "command_helpers.h"
#include "controller_config.h"

#define LOG_BUF_SIZE 2048

static char log_buf[LOG_BUF_SIZE];
static size_t log_pos = 0;

void controller_log_line(const char *fmt, ...)
{
    va_list args;
    va_start(args, fmt);

    int written = vsnprintf(
        log_buf + log_pos,
        LOG_BUF_SIZE - log_pos,
        fmt,
        args
    );

    va_end(args);

    if (written > 0) {
        log_pos += written;
        if (log_pos >= LOG_BUF_SIZE) {
            log_pos = LOG_BUF_SIZE - 1;
        }
    }
}

void controller_send_json(cJSON *obj)
{
    char *out = cJSON_PrintUnformatted(obj);
    uart_write_bytes(UART_NUM, out, strlen(out));
    uart_write_bytes(UART_NUM, "\n", 1);
    cJSON_free(out);
}

const char *controller_get_log_buffer(void)
{
    return log_buf;
}