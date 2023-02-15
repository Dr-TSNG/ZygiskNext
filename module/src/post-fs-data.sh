#!/system/bin/sh

MODDIR=${0%/*}

cd $MODDIR
export NATIVE_BRIDGE=$(getprop ro.dalvik.vm.native.bridge)

unshare -m sh -c "./daemon.sh $@&"
