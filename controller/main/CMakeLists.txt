idf_component_register(
    SRCS "controller.c" "cmd_goto.c" "cmd_ping.c" "cmd_track.c" "command_helpers.c" "setup_motors.c"
    INCLUDE_DIRS "."
    REQUIRES
        cjson
        esp_driver_uart
)