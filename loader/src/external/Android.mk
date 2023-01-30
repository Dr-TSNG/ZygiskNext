LOCAL_PATH := $(call my-dir)

# liblsplt.a
include $(CLEAR_VARS)
LOCAL_MODULE:= liblsplt
LOCAL_C_INCLUDES := $(LOCAL_PATH)/liblsplt/lsplt/src/main/jni/include
LOCAL_EXPORT_C_INCLUDES := $(LOCAL_C_INCLUDES)
LOCAL_CFLAGS := -Wall -Wextra -Werror -fvisibility=hidden -D__android_log_print=magisk_log_print
LOCAL_CPPFLAGS := -std=c++20
LOCAL_STATIC_LIBRARIES := libcxx
LOCAL_SRC_FILES := \
	liblsplt/lsplt/src/main/jni/elf_util.cc \
	liblsplt/lsplt/src/main/jni/lsplt.cc
include $(BUILD_STATIC_LIBRARY)
