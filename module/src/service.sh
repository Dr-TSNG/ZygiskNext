#!/system/bin/sh

DEBUG=@DEBUG@

MODDIR=${0%/*}
if [ "$ZYGISK_ENABLED" ]; then
  exit 0
fi

cd "$MODDIR"

# temporary fix AVD 11 magisk
if [ -f /dev/zygisk_service ];then
  log -p i -t "zygisk-sh" "service called twice";
  exit;
fi
touch /dev/zygisk_service

if [ "$(which magisk)" ]; then
  for file in ../*; do
    if [ -d "$file" ] && [ -d "$file/zygisk" ] && ! [ -f "$file/disable" ]; then
      if [ -f "$file/service.sh" ]; then
        cd "$file"
        log -p i -t "zygisk-sh" "Manually trigger service.sh for $file"
        sh "$(realpath ./service.sh)"
        cd "$MODDIR"
      fi
    fi
  done
fi

[ "$DEBUG" = true ] && export RUST_BACKTRACE=1
unshare -m sh -c "bin/zygisk-wd &"
