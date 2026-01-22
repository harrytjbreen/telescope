#pragma once

#include "cJSON.h"

void controller_log_line(const char *fmt, ...);
void controller_send_json(cJSON *obj);
const char *controller_get_log_buffer(void);