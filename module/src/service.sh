#!/system/bin/sh

DEBUG=@DEBUG@

MODDIR=${0%/*}
if [ "$ZYGISK_ENABLED" ]; then
  exit 0
fi

# temporary fix for AVD 30
if [ -f /dev/zygisk/wd ]; then
  log -p i -t "zygisk-sh" "prevent from instance duplicated"
  exit
fi
touch /dev/zygisk/wd

cd "$MODDIR"

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
unshare -m sh -c "bin/zygisk-cp64 &"
unshare -m sh -c "bin/zygisk-cp32 &"
