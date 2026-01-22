idf_component_register(
    SRCS "controller.c" "cmd_goto.c" "cmd_ping.c" "cmd_start.c" "cmd_stop.c" "command_helpers.c"
    INCLUDE_DIRS "."
    REQUIRES
        cjson
        esp_driver_uart
)