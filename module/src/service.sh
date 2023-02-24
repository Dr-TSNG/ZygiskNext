#!/system/bin/sh

MODDIR=${0%/*}
if [ "$ZYGISK_ENABLED" ]; then
  exit 0
fi

cd "$MODDIR"

if [ "$(which magisk)" ] && [ ".." -ef "/data/adb/modules" ]; then
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
