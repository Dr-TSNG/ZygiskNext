MODDIR=${0%/*}/..

if [ "$(which magisk)" ]; then
  export TMP_PATH="$(magisk --path)/zygisksu"
else
  export TMP_PATH="/debug_ramdisk/zygisksu"
fi

exec $MODDIR/bin/zygisk-ptrace64 ctl $*
