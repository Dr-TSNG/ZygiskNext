# shellcheck disable=SC2034
SKIPUNZIP=1

DEBUG=@DEBUG@
MIN_KSU_VERSION=@MIN_KSU_VERSION@
MIN_KSUD_VERSION=@MIN_KSUD_VERSION@
MAX_KSU_VERSION=@MAX_KSU_VERSION@
MIN_MAGISK_VERSION=@MIN_MAGISK_VERSION@

if [ "$BOOTMODE" ] && [ "$KSU" ]; then
  ui_print "- Installing from KernelSU app"
  ui_print "- KernelSU version: $KSU_KERNEL_VER_CODE (kernel) + $KSU_VER_CODE (ksud)"
  if ! [ "$KSU_KERNEL_VER_CODE" ] || [ "$KSU_KERNEL_VER_CODE" -lt "$MIN_KSU_VERSION" ]; then
    ui_print "*********************************************************"
    ui_print "! KernelSU version is too old!"
    ui_print "! Please update KernelSU to latest version"
    abort    "*********************************************************"
  elif [ "$KSU_KERNEL_VER_CODE" -ge "$MAX_KSU_VERSION" ]; then
    ui_print "*********************************************************"
    ui_print "! KernelSU version abnormal!"
    ui_print "! Please integrate KernelSU into your kernel"
    ui_print "  as submodule instead of copying the source code"
    abort    "*********************************************************"
  fi
  if ! [ "$KSU_VER_CODE" ] || [ "$KSU_VER_CODE" -lt "$MIN_KSUD_VERSION" ]; then
    ui_print "*********************************************************"
    ui_print "! ksud version is too old!"
    ui_print "! Please update KernelSU Manager to latest version"
    abort    "*********************************************************"
  fi
  if [ "$(which magisk)" ]; then
    ui_print "*********************************************************"
    ui_print "! Multiple root implementation is NOT supported!"
    ui_print "! Please uninstall Magisk before installing Zygisk Next"
    abort    "*********************************************************"
  fi
elif [ "$BOOTMODE" ] && [ "$MAGISK_VER_CODE" ]; then
  ui_print "- Installing from Magisk app"
  if [ "$MAGISK_VER_CODE" -lt "$MIN_MAGISK_VERSION" ]; then
    ui_print "*********************************************************"
    ui_print "! Magisk version is too old!"
    ui_print "! Please update Magisk to latest version"
    abort    "*********************************************************"
  fi
else
  ui_print "*********************************************************"
  ui_print "! Install from recovery is not supported"
  ui_print "! Please install from KernelSU or Magisk app"
  abort    "*********************************************************"
fi

VERSION=$(grep_prop version "${TMPDIR}/module.prop")
ui_print "- Installing Zygisk Next $VERSION"

# check android
if [ "$API" -lt 29 ]; then
  ui_print "! Unsupported sdk: $API"
  abort "! Minimal supported sdk is 29 (Android 10)"
else
  ui_print "- Device sdk: $API"
fi

# check architecture
if [ "$ARCH" != "arm" ] && [ "$ARCH" != "arm64" ] && [ "$ARCH" != "x86" ] && [ "$ARCH" != "x64" ]; then
  abort "! Unsupported platform: $ARCH"
else
  ui_print "- Device platform: $ARCH"
fi

ui_print "- Extracting verify.sh"
unzip -o "$ZIPFILE" 'verify.sh' -d "$TMPDIR" >&2
if [ ! -f "$TMPDIR/verify.sh" ]; then
  ui_print "*********************************************************"
  ui_print "! Unable to extract verify.sh!"
  ui_print "! This zip may be corrupted, please try downloading again"
  abort    "*********************************************************"
fi
. "$TMPDIR/verify.sh"
extract "$ZIPFILE" 'customize.sh'  "$TMPDIR/.vunzip"
extract "$ZIPFILE" 'verify.sh'     "$TMPDIR/.vunzip"
extract "$ZIPFILE" 'sepolicy.rule' "$TMPDIR"

if [ "$DEBUG" = true ]; then
  ui_print "- Add debug SELinux policy"
  echo "allow crash_dump adb_data_file dir search" >> "$TMPDIR/sepolicy.rule"
fi

if [ "$KSU" ]; then
  ui_print "- Checking SELinux patches"
  if ! check_sepolicy "$TMPDIR/sepolicy.rule"; then
    ui_print "*********************************************************"
    ui_print "! Unable to apply SELinux patches!"
    ui_print "! Your kernel may not support SELinux patch fully"
    abort    "*********************************************************"
  fi
fi

ui_print "- Extracting module files"
extract "$ZIPFILE" 'module.prop'     "$MODPATH"
extract "$ZIPFILE" 'post-fs-data.sh' "$MODPATH"
extract "$ZIPFILE" 'service.sh'      "$MODPATH"
mv "$TMPDIR/sepolicy.rule" "$MODPATH"

HAS32BIT=false && [ -d "/system/lib" ] && HAS32BIT=true

mkdir "$MODPATH/bin"
mkdir "$MODPATH/system"
mkdir "$MODPATH/system/lib64"
[ "$HAS32BIT" = true ] && mkdir "$MODPATH/system/lib"

if [ "$ARCH" = "x86" ] || [ "$ARCH" = "x64" ]; then
  if [ "$HAS32BIT" = true ]; then
    ui_print "- Extracting x86 libraries"
    extract "$ZIPFILE" 'bin/x86/zygiskd' "$MODPATH/bin" true
    mv "$MODPATH/bin/zygiskd" "$MODPATH/bin/zygiskd32"
    extract "$ZIPFILE" 'lib/x86/libzygisk.so' "$MODPATH/system/lib" true
    ln -sf "zygiskd32" "$MODPATH/bin/zygisk-cp32"
    ln -sf "zygiskd32" "$MODPATH/bin/zygisk-ptrace32"
  fi

  ui_print "- Extracting x64 libraries"
  extract "$ZIPFILE" 'bin/x86_64/zygiskd' "$MODPATH/bin" true
  mv "$MODPATH/bin/zygiskd" "$MODPATH/bin/zygiskd64"
  extract "$ZIPFILE" 'lib/x86_64/libzygisk.so' "$MODPATH/system/lib64" true
  ln -sf "zygiskd64" "$MODPATH/bin/zygisk-wd"
  ln -sf "zygiskd64" "$MODPATH/bin/zygisk-fuse"
  ln -sf "zygiskd64" "$MODPATH/bin/zygisk-cp64"
  ln -sf "zygiskd64" "$MODPATH/bin/zygisk-ptrace64"
else
  if [ "$HAS32BIT" = true ]; then
    ui_print "- Extracting arm libraries"
    extract "$ZIPFILE" 'bin/armeabi-v7a/zygiskd' "$MODPATH/bin" true
    mv "$MODPATH/bin/zygiskd" "$MODPATH/bin/zygiskd32"
    extract "$ZIPFILE" 'lib/armeabi-v7a/libzygisk.so' "$MODPATH/system/lib" true
    ln -sf "zygiskd32" "$MODPATH/bin/zygisk-cp32"
    ln -sf "zygiskd32" "$MODPATH/bin/zygisk-ptrace32"
  fi

  ui_print "- Extracting arm64 libraries"
  extract "$ZIPFILE" 'bin/arm64-v8a/zygiskd' "$MODPATH/bin" true
  mv "$MODPATH/bin/zygiskd" "$MODPATH/bin/zygiskd64"
  extract "$ZIPFILE" 'lib/arm64-v8a/libzygisk.so' "$MODPATH/system/lib64" true
  ln -sf "zygiskd64" "$MODPATH/bin/zygisk-wd"
  ln -sf "zygiskd64" "$MODPATH/bin/zygisk-fuse"
  ln -sf "zygiskd64" "$MODPATH/bin/zygisk-cp64"
  ln -sf "zygiskd64" "$MODPATH/bin/zygisk-ptrace64"
fi

ui_print "- Setting permissions"
set_perm_recursive "$MODPATH/bin" 0 0 0755 0755
set_perm_recursive "$MODPATH/system/lib" 0 0 0755 0644 u:object_r:system_lib_file:s0
set_perm_recursive "$MODPATH/system/lib64" 0 0 0755 0644 u:object_r:system_lib_file:s0

# If Huawei's Maple is enabled, system_server is created with a special way which is out of Zygisk's control
HUAWEI_MAPLE_ENABLED=$(grep_prop ro.maple.enable)
if [ "$HUAWEI_MAPLE_ENABLED" == "1" ]; then
  ui_print "- Add ro.maple.enable=0"
  echo "ro.maple.enable=0" >>"$MODPATH/system.prop"
fi
