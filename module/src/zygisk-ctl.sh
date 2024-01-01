MODDIR=${0%/*}/..
export MAGIC=$(cat /data/adb/zygisksu/magic)
exec $MODDIR/bin/zygisk-ptrace64 ctl $*
