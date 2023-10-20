LOCAL_PATH := $(call my-dir)
define walk
  $(wildcard $(1)) $(foreach e, $(wildcard $(1)/*), $(call walk, $(e)))
endef

include $(CLEAR_VARS)
LOCAL_MODULE           := common
LOCAL_C_INCLUDES       := $(LOCAL_PATH)/include
FILE_LIST              := $(filter %.cpp, $(call walk, $(LOCAL_PATH)/common))
LOCAL_SRC_FILES        := $(FILE_LIST:COMMON_FILE_LIST:$(LOCAL_PATH)/%=%)
LOCAL_STATIC_LIBRARIES := cxx
LOCAL_LDLIBS           := -llog
include $(BUILD_STATIC_LIBRARY)

include $(CLEAR_VARS)
LOCAL_MODULE           := zygisk
LOCAL_C_INCLUDES       := $(LOCAL_PATH)/include
FILE_LIST              := $(filter %.cpp, $(call walk, $(LOCAL_PATH)/injector))
LOCAL_SRC_FILES        := $(FILE_LIST:COMMON_FILE_LIST:$(LOCAL_PATH)/%=%)
LOCAL_STATIC_LIBRARIES := cxx common liblsplt libphmap
LOCAL_LDLIBS           := -llog
include $(BUILD_SHARED_LIBRARY)

$(call import-module,prefab/cxx)

include src/external/Android.mk
