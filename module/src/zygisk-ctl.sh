MODDIR=${0%/*}/..

TMP_PATH=/sbin
[ -f /sbin ] || TMP_PATH=/debug_ramdisk

exec $MODDIR/bin/zygisk-ptrace64 ctl $*
