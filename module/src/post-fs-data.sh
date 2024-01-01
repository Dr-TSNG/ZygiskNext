#!/system/bin/sh

MODDIR=${0%/*}
if [ "$ZYGISK_ENABLED" ]; then
  exit 0
fi

cd "$MODDIR"

MAGIC=$(cat /data/adb/zygisksu/magic)
MAGIC_PATH=/dev/zygisk_$MAGIC
export MAGIC
export MAGIC_PATH

if [ "$(which magisk)" ]; then
  for file in ../*; do
    if [ -d "$file" ] && [ -d "$file/zygisk" ] && ! [ -f "$file/disable" ]; then
      if [ -f "$file/post-fs-data.sh" ]; then
        cd "$file"
        log -p i -t "zygisk-sh" "Manually trigger post-fs-data.sh for $file"
        sh "$(realpath ./post-fs-data.sh)"
        cd "$MODDIR"
      fi
    fi
  done
fi

create_sys_perm() {
  mkdir -p $1
  chmod 555 $1
  chcon u:object_r:system_file:s0 $1
}

create_sys_perm $MAGIC_PATH

if [ -f $MODDIR/lib64/libzygisk.so ];then
  create_sys_perm $MAGIC_PATH/lib64
  cp $MODDIR/lib64/libzygisk.so $MAGIC_PATH/lib64/libzygisk.so
  chcon u:object_r:system_file:s0 $MAGIC_PATH/lib64/libzygisk.so
fi

if [ -f $MODDIR/lib/libzygisk.so ];then
  create_sys_perm $MAGIC_PATH/lib
  cp $MODDIR/lib/libzygisk.so $MAGIC_PATH/lib/libzygisk.so
  chcon u:object_r:system_file:s0 $MAGIC_PATH/lib/libzygisk.so
fi

[ "$DEBUG" = true ] && export RUST_BACKTRACE=1
unshare -m sh -c "./bin/zygisk-ptrace64 monitor &"
