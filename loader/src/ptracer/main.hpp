#pragma once


void init_monitor();
bool trace_zygote(int pid);

enum Command {
    START = 1,
    STOP,
    EXIT
};

void send_control_command(Command cmd);
