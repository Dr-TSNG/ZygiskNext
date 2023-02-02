# Zygisk on KernelSU

Zygisk loader for KernelSU, which allows Zygisk modules to run without Magisk environment.

## Development road map

- [x] Basic Zygisk loader
- [x] Stabilize injector
- [ ] Separate zygiskd process
- [ ] Handle 64 bit only devices
- [ ] DenyList

## Running on Magisk

It is possible to run Zygisksu on Magisk with a few steps:

1. `mkdir /data/adb/ksu`
2. `ln -s /data/adb/modules /data/adb/ksu/modules`
3. `cp $(which resetprop) /data/adb/ksu/resetprop`
