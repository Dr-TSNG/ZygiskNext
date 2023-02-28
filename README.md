# Zygisk on KernelSU

Zygisk loader for KernelSU, allowing Zygisk modules to run without Magisk environment.

Also works as standalone loader for Magisk on purpose of getting rid of LD_PRELOAD.

## Requirements

### General

+ No multiple root implementation installed

### KernelSU

+ Minimal KernelSU version: 10654
+ Minimal ksud version: 10647
+ Kernel has full SELinux patch support
+ For old kernels, you may need to manually add the following code to `sepolicy.rule`:  
  `allow zygote appdomain_tmpfs file *`  
  `allow zygote appdomain_tmpfs dir *`

### Magisk

+ Minimal version: 25208
+ Original Zygisk turned off

## Compatibility

Should work with everything except those rely on Magisk internal behaviors.
