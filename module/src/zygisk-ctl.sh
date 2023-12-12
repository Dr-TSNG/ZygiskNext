MODDIR=${0%/*}/..
export MAGIC=$(cat $MODDIR/magic)
exec $MODDIR/bin/zygisk-ptrace64 ctl $*
