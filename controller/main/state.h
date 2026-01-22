#pragma once
#include <stdbool.h>

typedef struct {
    double ra;
    double dec;
} target_t;

typedef enum {
    MOUNT_IDLE,
    MOUNT_SLEWING,
    MOUNT_TRACKING,
    MOUNT_ERROR
} mount_status_t;

typedef struct {
    mount_status_t status;

    bool gps_fix;
    double latitude;
    double longitude;

    target_t current;
    target_t target;
} app_state_t;