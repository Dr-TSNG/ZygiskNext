#!/system/bin/sh

DEBUG=@DEBUG@

# shellcheck disable=SC2155
export NATIVE_BRIDGE=$(getprop ro.dalvik.vm.native.bridge)
[ "$DEBUG" = true ] && export RUST_BACKTRACE=1

log -p i -t "zygisksu" "Start watchdog"
resetprop ro.dalvik.vm.native.bridge libzygiskloader.so
exec "bin/zygiskwd" "watchdog" >/dev/null 2>&1
