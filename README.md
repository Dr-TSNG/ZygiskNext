# Zygisk Next

Standalone implementation of Zygisk, providing Zygisk API support for KernelSU and a replacement of Magisk's built-in Zygisk.

## Requirements

### General

+ No multiple root implementation installed

### KernelSU

+ Minimal KernelSU version: 10940
+ Minimal ksud version: 10942
+ Kernel has full SELinux patch support

### Magisk

+ Minimal version: 26300
+ Built-in Zygisk turned off

## Compatibility

`PROCESS_ON_DENYLIST` cannot be flagged correctly for isolated processes on Magisk DenyList currently.

Zygisk Next only guarantees the same behavior of Zygisk API, but will NOT ensure Magisk's internal features.
