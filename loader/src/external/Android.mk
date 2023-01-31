LOCAL_PATH := $(call my-dir)

# liblsplt.a
include $(CLEAR_VARS)
LOCAL_MODULE:= liblsplt
LOCAL_C_INCLUDES := $(LOCAL_PATH)/liblsplt/lsplt/src/main/jni/include
LOCAL_EXPORT_C_INCLUDES := $(LOCAL_C_INCLUDES)
LOCAL_CFLAGS := -Wall -Wextra -Werror -fvisibility=hidden
LOCAL_CPPFLAGS := -std=c++20
LOCAL_STATIC_LIBRARIES := libcxx
LOCAL_SRC_FILES := \
	liblsplt/lsplt/src/main/jni/elf_util.cc \
	liblsplt/lsplt/src/main/jni/lsplt.cc
include $(BUILD_STATIC_LIBRARY)

# Header only library
include $(CLEAR_VARS)
LOCAL_MODULE:= libphmap
LOCAL_EXPORT_C_INCLUDES := $(LOCAL_PATH)/parallel-hashmap
include $(BUILD_STATIC_LIBRARY)
