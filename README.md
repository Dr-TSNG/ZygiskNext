# Zygisk on KernelSU

Zygisk loader for KernelSU, allowing Zygisk modules to run without Magisk environment.

Also works as standalone loader for Magisk on purpose of getting rid of LD_PRELOAD. (Coming soon)

## Requirements

+ Minimal KernelSU version: 10575
+ Minimal ksud version: 10616
+ Full SELinux patch support (If non-gki kernel)

## Compatibility

Should work with everything except those rely on Magisk internal behaviors.

## Development road map

- [x] [Inject] Basic Zygisk loader
- [x] [Inject] Stabilize injector
- [x] [Inject] Unload
- [x] [Daemon] Linker namespace
- [x] [Daemon] Separate zygiskd process
- [x] [Daemon] Handle 64 bit only devices
- [x] [Daemon] Handle zygote death
- [ ] [ Misc ] Support Magisk out of box
