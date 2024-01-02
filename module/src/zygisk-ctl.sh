MODDIR=${0%/*}/..
exec $MODDIR/bin/zygisk-ptrace64 ctl $*
