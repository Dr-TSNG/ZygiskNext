#pragma once

#include "jni_helper.hpp"

template <typename T>
constexpr inline auto RoundUpTo(T v, size_t size) {
    return v + size - 1 - ((v + size - 1) & (size - 1));
}

inline static constexpr auto kPointerSize = sizeof(void *);

namespace lsplant::art {

    class ArtMethod {

    public:
        void *GetData() {
            return *reinterpret_cast<void **>(reinterpret_cast<uintptr_t>(this) + data_offset);
        }

        static art::ArtMethod *FromReflectedMethod(JNIEnv *env, jobject method) {
            if (art_method_field) [[likely]] {
                return reinterpret_cast<art::ArtMethod *>(
                        JNI_GetLongField(env, method, art_method_field));
            } else {
                return reinterpret_cast<art::ArtMethod *>(env->FromReflectedMethod(method));
            }
        }

        static bool Init(JNIEnv *env) {
            ScopedLocalRef<jclass> executable{env, nullptr};
            executable = JNI_FindClass(env, "java/lang/reflect/Executable");
            if (!executable) {
                LOGE("Failed to found Executable");
                return false;
            }

            if (art_method_field = JNI_GetFieldID(env, executable, "artMethod", "J");
                    !art_method_field) {
                LOGE("Failed to find artMethod field");
                return false;
            }

            auto throwable = JNI_FindClass(env, "java/lang/Throwable");
            if (!throwable) {
                LOGE("Failed to found Executable");
                return false;
            }
            auto clazz = JNI_FindClass(env, "java/lang/Class");
            static_assert(std::is_same_v<decltype(clazz)::BaseType, jclass>);
            jmethodID get_declared_constructors = JNI_GetMethodID(env, clazz, "getDeclaredConstructors",
                                                                  "()[Ljava/lang/reflect/Constructor;");
            const auto constructors =
                    JNI_Cast<jobjectArray>(JNI_CallObjectMethod(env, throwable, get_declared_constructors));
            if (constructors.size() < 2) {
                LOGE("Throwable has less than 2 constructors");
                return false;
            }
            auto &first_ctor = constructors[0];
            auto &second_ctor = constructors[1];
            auto *first = FromReflectedMethod(env, first_ctor.get());
            auto *second = FromReflectedMethod(env, second_ctor.get());
            art_method_size = reinterpret_cast<uintptr_t>(second) - reinterpret_cast<uintptr_t>(first);
            LOGD("ArtMethod size: %zu", art_method_size);
            if (RoundUpTo(4 * 9, kPointerSize) + kPointerSize * 3 < art_method_size) [[unlikely]] {
                LOGW("ArtMethod size exceeds maximum assume. There may be something wrong.");
            }
            entry_point_offset = art_method_size - kPointerSize;
            data_offset = entry_point_offset - kPointerSize;
            LOGD("ArtMethod::entrypoint offset: %zu", entry_point_offset);
            LOGD("ArtMethod::data offset: %zu", data_offset);
            return true;
        }

    private:
        inline static jfieldID art_method_field = nullptr;
        inline static size_t art_method_size = 0;
        inline static size_t entry_point_offset = 0;
        inline static size_t data_offset = 0;
    };

}  // namespace lsplant::art
