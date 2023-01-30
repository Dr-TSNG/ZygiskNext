#!/system/bin/sh

MODDIR=${0%/*}
export NATIVE_BRIDGE=$(getprop ro.dalvik.vm.native.bridge)
unshare -m sh -c "$MODDIR/daemon.sh $@&"
