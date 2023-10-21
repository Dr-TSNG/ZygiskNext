# Zygisk on KernelSU

Zygisk loader for KernelSU, allowing Zygisk modules to run without Magisk environment.

Also works as standalone loader for Magisk.

## Requirements

### General

+ No multiple root implementation installed
+ SELinux enforcing: We now rely on SELinux to prevent `vold` from aborting our fuse connection

### KernelSU

+ Minimal KernelSU version: 10940
+ Minimal ksud version: 10942
+ Kernel has full SELinux patch support

### Magisk

+ Minimal version: 26300
+ Original Zygisk turned off

## Compatibility

Should work with everything except those rely on Magisk internal behaviors.
