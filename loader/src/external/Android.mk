LOCAL_PATH := $(call my-dir)

# liblsplt.a
include $(CLEAR_VARS)
LOCAL_MODULE:= liblsplt
LOCAL_C_INCLUDES := $(LOCAL_PATH)/lsplt/lsplt/src/main/jni/include
LOCAL_EXPORT_C_INCLUDES := $(LOCAL_C_INCLUDES)
LOCAL_CFLAGS := -Wall -Wextra -Werror -fvisibility=hidden -DLOG_DISABLED
LOCAL_CPPFLAGS := -std=c++20
LOCAL_STATIC_LIBRARIES := libcxx
LOCAL_SRC_FILES := \
    lsplt/lsplt/src/main/jni/elf_util.cc \
    lsplt/lsplt/src/main/jni/lsplt.cc
include $(BUILD_STATIC_LIBRARY)

# Header only library
include $(CLEAR_VARS)
LOCAL_MODULE:= libphmap
LOCAL_CFLAGS := -Wno-unused-value
LOCAL_EXPORT_C_INCLUDES := $(LOCAL_PATH)/parallel-hashmap
include $(BUILD_STATIC_LIBRARY)
