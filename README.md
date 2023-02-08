# Zygisk on KernelSU

Zygisk loader for KernelSU, allowing Zygisk modules to run without Magisk environment.

Warning: The current version of Zygisksu is UNSTABLE. You may suffer boot loop or even data loss so use with caution.

## Requirements

+ Minimal KernelSU version: 10575
+ Minimal ksud version: 10200
+ Full SELinux patch support (If non-gki kernel)

## Compatibility

- [x] LSPosed
- [x] Storage Isolation
- [ ] IFW Enhance
- [ ] Universal SafetyNet Fix
- [ ] Shamiko

## Development road map

- [x] [Inject] Basic Zygisk loader
- [x] [Inject] Stabilize injector
- [x] [Inject] Unload
- [x] [Daemon] Linker namespace
- [ ] [Daemon] Separate zygiskd process
- [ ] [Daemon] Handle 64 bit only devices
- [ ] [Daemon] Handle zygote death

## Running on Magisk

It is possible to run Zygisksu on Magisk with a few steps:

1. `mkdir /data/adb/ksu`
2. `ln -s /data/adb/modules /data/adb/ksu/`
