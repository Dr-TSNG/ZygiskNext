#!/system/bin/sh

MODDIR=${0%/*}
if [ "$ZYGISK_ENABLED" ]; then
  exit 0
fi

cd "$MODDIR"

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

create_sys_perm /dev/zygisk

if [ -f $MODDIR/lib64/libzygisk.so ];then
  create_sys_perm /dev/zygisk/lib64
  cp $MODDIR/lib64/libzygisk.so /dev/zygisk/lib64/libzygisk.so
  chcon u:object_r:system_lib_file:s0 /dev/zygisk/lib64/libzygisk.so
  setprop ctl.sigstop_on zygote
  unshare -m sh -c "./bin/zygisk-ptracer64 prop_monitor &"
fi

if [ -f $MODDIR/lib/libzygisk.so ];then
  create_sys_perm /dev/zygisk/lib
  cp $MODDIR/lib/libzygisk.so /dev/zygisk/lib/libzygisk.so
  chcon u:object_r:system_lib_file:s0 /dev/zygisk/lib/libzygisk.so
  setprop ctl.sigstop_on zygote_secondary
  unshare -m sh -c "./bin/zygisk-ptracer32 prop_monitor &"
fi

