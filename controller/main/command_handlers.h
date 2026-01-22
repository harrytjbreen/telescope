#pragma once

#include "cJSON.h"

void handle_cmd_ping(cJSON *root);
void handle_cmd_start(cJSON *root);
void handle_cmd_stop(cJSON *root);
void handle_cmd_goto(cJSON *root);