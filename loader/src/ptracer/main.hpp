#pragma once


void init_monitor();
bool trace_zygote(int pid);

enum Command {
    START = 1,
    STOP = 2,
    EXIT = 3,
    // sent from daemon
    ZYGOTE64_INJECTED = 4,
    ZYGOTE32_INJECTED = 5
};

void send_control_command(Command cmd);
