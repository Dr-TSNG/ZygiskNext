#!/system/bin/sh

DEBUG=@DEBUG@

MODDIR=${0%/*}
if [ "$ZYGISK_ENABLED" ]; then
  exit 0
fi

cd "$MODDIR"

if [ "$(which magisk)" ]; then
  for file in ../*; do
    if [ -d "$file" ] && [ -d "$file/zygisk" ] && ! [ -f "$file/disable" ]; then
      if [ -f "$file/service.sh" ]; then
        cd "$file"
        log -p i -t "zygisksu" "Manually trigger service.sh for $file"
        sh "$(realpath ./service.sh)"
        cd "$MODDIR"
      fi
    fi
  done
fi

log -p i -t "zygisksu" "Start watchdog"
[ "$DEBUG" = true ] && export RUST_BACKTRACE=1
exec "bin/zygisk-wd" >/dev/null 2>&1
