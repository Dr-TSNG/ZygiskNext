# Zygisk on KernelSU

Zygisk loader for KernelSU, allowing Zygisk modules to run without Magisk environment.

Also works as standalone loader for Magisk.

## Requirements

### General

+ No multiple root implementation installed

### KernelSU

+ Minimal KernelSU version: 10940
+ Minimal ksud version: 10942
+ Kernel has full SELinux patch support

### Magisk

+ Minimal version: 25208
+ Original Zygisk turned off

## Compatibility

Should work with everything except those rely on Magisk internal behaviors.
