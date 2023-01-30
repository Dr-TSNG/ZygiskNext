# Zygisk on KernelSU

## Running on Magisk

It is possible to run Zygisksu on Magisk with a few steps:

1. `mkdir /data/adb/ksu`
2. `ln -s /data/adb/modules /data/adb/ksu/modules`
3. `cp $(which resetprop) /data/adb/ksu/resetprop`
