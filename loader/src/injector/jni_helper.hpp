#pragma once

#include <android/log.h>
#include <jni.h>

#include <string>
#include <string_view>

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Winvalid-partial-specialization"
#pragma clang diagnostic ignored "-Wunknown-pragmas"
#pragma ide diagnostic ignored "OCUnusedGlobalDeclarationInspection"

#define DISALLOW_COPY_AND_ASSIGN(TypeName)                                                         \
    TypeName(const TypeName &) = delete;                                                           \
    void operator=(const TypeName &) = delete

namespace lsplant {
    template <class, template <class, class...> class>
    struct is_instance : public std::false_type {};

    template <class... Ts, template <class, class...> class U>
    struct is_instance<U<Ts...>, U> : public std::true_type {};

    template <class T, template <class, class...> class U>
    inline constexpr bool is_instance_v = is_instance<T, U>::value;

    template <typename T>
    concept JObject = std::is_base_of_v<std::remove_pointer_t<_jobject>, std::remove_pointer_t<T>>;

    template <JObject T>
    class ScopedLocalRef {
    public:
        using BaseType [[maybe_unused]] = T;

        ScopedLocalRef(JNIEnv *env, T local_ref) : env_(env), local_ref_(nullptr) { reset(local_ref); }

        ScopedLocalRef(ScopedLocalRef &&s) noexcept : ScopedLocalRef(s.env_, s.release()) {}

        template <JObject U>
        ScopedLocalRef(ScopedLocalRef<U> &&s) noexcept : ScopedLocalRef(s.env_, (T)s.release()) {}

        explicit ScopedLocalRef(JNIEnv *env) noexcept : ScopedLocalRef(env, T{nullptr}) {}

        ~ScopedLocalRef() { reset(); }

        void reset(T ptr = nullptr) {
            if (ptr != local_ref_) {
                if (local_ref_ != nullptr) {
                    env_->DeleteLocalRef(local_ref_);
                }
                local_ref_ = ptr;
            }
        }

        [[nodiscard]] T release() {
            T localRef = local_ref_;
            local_ref_ = nullptr;
            return localRef;
        }

        T get() const { return local_ref_; }

        operator T() const { return local_ref_; }

        // We do not expose an empty constructor as it can easily lead to errors
        // using common idioms, e.g.:
        //   ScopedLocalRef<...> ref;
        //   ref.reset(...);
        // Move assignment operator.
        ScopedLocalRef &operator=(ScopedLocalRef &&s) noexcept {
            reset(s.release());
            env_ = s.env_;
            return *this;
        }

        operator bool() const { return local_ref_; }

        template <JObject U>
        friend class ScopedLocalRef;

        friend class JUTFString;

    private:
        JNIEnv *env_;
        T local_ref_;
        DISALLOW_COPY_AND_ASSIGN(ScopedLocalRef);
    };

    template <typename T>
    concept JArray = std::is_base_of_v<std::remove_pointer_t<_jarray>, std::remove_pointer_t<T>>;

    template <JArray T>
    class ScopedLocalRef<T>;

    class JNIScopeFrame {
        JNIEnv *env_;

        DISALLOW_COPY_AND_ASSIGN(JNIScopeFrame);

    public:
        JNIScopeFrame(JNIEnv *env, jint size) : env_(env) { env_->PushLocalFrame(size); }

        ~JNIScopeFrame() { env_->PopLocalFrame(nullptr); }
    };

    class JNIMonitor {
        JNIEnv *env_;
        jobject obj_;

        DISALLOW_COPY_AND_ASSIGN(JNIMonitor);

    public:
        JNIMonitor(JNIEnv *env, jobject obj) : env_(env), obj_(obj) { env_->MonitorEnter(obj_); }

        ~JNIMonitor() { env_->MonitorExit(obj_); }
    };

    template <typename T, typename U>
    concept ScopeOrRaw = std::is_convertible_v<T, U> ||
    (is_instance_v<std::decay_t<T>, ScopedLocalRef>
    &&std::is_convertible_v<typename std::decay_t<T>::BaseType, U>);

    template <typename T>
    concept ScopeOrClass = ScopeOrRaw<T, jclass>;

    template <typename T>
    concept ScopeOrObject = ScopeOrRaw<T, jobject>;

    inline ScopedLocalRef<jstring> ClearException(JNIEnv *env) {
        if (auto exception = env->ExceptionOccurred()) {
            env->ExceptionClear();
            static jclass log = (jclass)env->NewGlobalRef(env->FindClass("android/util/Log"));
            static jmethodID toString = env->GetStaticMethodID(
                    log, "getStackTraceString", "(Ljava/lang/Throwable;)Ljava/lang/String;");
            auto str = (jstring)env->CallStaticObjectMethod(log, toString, exception);
            env->DeleteLocalRef(exception);
            return {env, str};
        }
        return {env, nullptr};
    }

    template <typename T>
    [[maybe_unused]] inline auto UnwrapScope(T &&x) {
        if constexpr (std::is_same_v<std::decay_t<T>, std::string_view>)
        return x.data();
        else if constexpr (is_instance_v<std::decay_t<T>, ScopedLocalRef>)
        return x.get();
        else
        return std::forward<T>(x);
    }

    template <typename T>
    [[maybe_unused]] inline auto WrapScope(JNIEnv *env, T &&x) {
        if constexpr (std::is_convertible_v<T, _jobject *>) {
            return ScopedLocalRef(env, std::forward<T>(x));
        } else
        return x;
    }

    template <typename... T, size_t... I>
    [[maybe_unused]] inline auto WrapScope(JNIEnv *env, std::tuple<T...> &&x,
                                           std::index_sequence<I...>) {
        return std::make_tuple(WrapScope(env, std::forward<T>(std::get<I>(x)))...);
    }

    template <typename... T>
    [[maybe_unused]] inline auto WrapScope(JNIEnv *env, std::tuple<T...> &&x) {
        return WrapScope(env, std::forward<std::tuple<T...>>(x),
                std::make_index_sequence<sizeof...(T)>());
    }

    inline auto JNI_NewStringUTF(JNIEnv *env, std::string_view sv) {
        return ScopedLocalRef(env, env->NewStringUTF(sv.data()));
    }

    class JUTFString {
    public:
        inline JUTFString(JNIEnv *env, jstring jstr) : JUTFString(env, jstr, nullptr) {}

        inline JUTFString(const ScopedLocalRef<jstring> &jstr)
                : JUTFString(jstr.env_, jstr.local_ref_, nullptr) {}

        inline JUTFString(JNIEnv *env, jstring jstr, const char *default_cstr)
                : env_(env), jstr_(jstr) {
            if (env_ && jstr_)
                cstr_ = env_->GetStringUTFChars(jstr, nullptr);
            else
                cstr_ = default_cstr;
        }

        inline operator const char *() const { return cstr_; }

        inline operator const std::string() const { return cstr_; }

        inline operator const bool() const { return cstr_ != nullptr; }

        inline auto get() const { return cstr_; }

        inline ~JUTFString() {
            if (env_ && jstr_) env_->ReleaseStringUTFChars(jstr_, cstr_);
        }

        JUTFString(JUTFString &&other)
                : env_(std::move(other.env_)),
                  jstr_(std::move(other.jstr_)),
                  cstr_(std::move(other.cstr_)) {
            other.cstr_ = nullptr;
        }

        JUTFString &operator=(JUTFString &&other) {
            if (&other != this) {
                env_ = std::move(other.env_);
                jstr_ = std::move(other.jstr_);
                cstr_ = std::move(other.cstr_);
                other.cstr_ = nullptr;
            }
            return *this;
        }

    private:
        JNIEnv *env_;
        jstring jstr_;
        const char *cstr_;

        JUTFString(const JUTFString &) = delete;

        JUTFString &operator=(const JUTFString &) = delete;
    };

    template <typename Func, typename... Args>
    requires(std::is_function_v<Func>)
    [[maybe_unused]] inline auto JNI_SafeInvoke(JNIEnv *env, Func JNIEnv::*f, Args &&...args) {
        struct finally {
            finally(JNIEnv *env) : env_(env) {}

            ~finally() {
                if (auto exception = ClearException(env_)) {
                    __android_log_print(ANDROID_LOG_ERROR,
#ifdef LOG_TAG
                            LOG_TAG,
#else
                                        "JNIHelper",
#endif
                                        "%s", JUTFString(env_, exception.get()).get());
                }
            }

            JNIEnv *env_;
        } _(env);

        if constexpr (!std::is_same_v<void,
                      std::invoke_result_t<Func, decltype(UnwrapScope(
                            std::forward<Args>(args)))...>>)
        return WrapScope(env, (env->*f)(UnwrapScope(std::forward<Args>(args))...));
        else
        (env->*f)(UnwrapScope(std::forward<Args>(args))...);
    }

// functions to class

    [[maybe_unused]] inline auto JNI_FindClass(JNIEnv *env, std::string_view name) {
        return JNI_SafeInvoke(env, &JNIEnv::FindClass, name);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_GetObjectClass(JNIEnv *env, const Object &obj) {
        return JNI_SafeInvoke(env, &JNIEnv::GetObjectClass, obj);
    }

// functions to field

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetFieldID(JNIEnv *env, Class &&clazz, std::string_view name,
                                                std::string_view sig) {
        return JNI_SafeInvoke(env, &JNIEnv::GetFieldID, std::forward<Class>(clazz), name, sig);
    }

// getters

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_GetObjectField(JNIEnv *env, Object &&obj, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetObjectField, std::forward<Object>(obj), fieldId);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_GetBooleanField(JNIEnv *env, Object &&obj, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetBooleanField, std::forward<Object>(obj), fieldId);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_GetByteField(JNIEnv *env, Object &&obj, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetByteField, std::forward<Object>(obj), fieldId);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_GetCharField(JNIEnv *env, Object &&obj, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetCharField, std::forward<Object>(obj), fieldId);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_GetShortField(JNIEnv *env, Object &&obj, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetShortField, std::forward<Object>(obj), fieldId);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_GetIntField(JNIEnv *env, Object &&obj, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetIntField, std::forward<Object>(obj), fieldId);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_GetLongField(JNIEnv *env, Object &&obj, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetLongField, std::forward<Object>(obj), fieldId);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_GetFloatField(JNIEnv *env, Object &&obj, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetFloatField, std::forward<Object>(obj), fieldId);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_GetDoubleField(JNIEnv *env, Object &&obj, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetDoubleField, std::forward<Object>(obj), fieldId);
    }

// setters

    template <ScopeOrObject Object, ScopeOrObject Value>
    [[maybe_unused]] inline auto JNI_SetObjectField(JNIEnv *env, Object &&obj, jfieldID fieldId,
                                                    const Value &value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetObjectField, std::forward<Object>(obj), fieldId, value);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_SetBooleanField(JNIEnv *env, Object &&obj, jfieldID fieldId,
                                                     jboolean value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetBooleanField, std::forward<Object>(obj), fieldId, value);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_SetByteField(JNIEnv *env, Object &&obj, jfieldID fieldId,
                                                  jbyte value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetByteField, std::forward<Object>(obj), fieldId, value);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_SetCharField(JNIEnv *env, Object &&obj, jfieldID fieldId,
                                                  jchar value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetCharField, std::forward<Object>(obj), fieldId, value);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_SetShortField(JNIEnv *env, Object &&obj, jfieldID fieldId,
                                                   jshort value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetShortField, std::forward<Object>(obj), fieldId, value);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_SetIntField(JNIEnv *env, Object &&obj, jfieldID fieldId,
                                                 jint value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetIntField, std::forward<Object>(obj), fieldId, value);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_SetLongField(JNIEnv *env, Object &&obj, jfieldID fieldId,
                                                  jlong value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetLongField, std::forward<Object>(obj), fieldId, value);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_SetFloatField(JNIEnv *env, Object &&obj, jfieldID fieldId,
                                                   jfloat value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetFloatField, std::forward<Object>(obj), fieldId, value);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_SetDoubleField(JNIEnv *env, Object &&obj, jfieldID fieldId,
                                                    jdouble value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetDoubleField, std::forward<Object>(obj), fieldId, value);
    }

// functions to static field

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetStaticFieldID(JNIEnv *env, Class &&clazz, std::string_view name,
                                                      std::string_view sig) {
        return JNI_SafeInvoke(env, &JNIEnv::GetStaticFieldID, std::forward<Class>(clazz), name, sig);
    }

// getters

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetStaticObjectField(JNIEnv *env, Class &&clazz,
                                                          jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetStaticObjectField, std::forward<Class>(clazz), fieldId);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetStaticBooleanField(JNIEnv *env, Class &&clazz,
                                                           jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetStaticBooleanField, std::forward<Class>(clazz), fieldId);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetStaticByteField(JNIEnv *env, Class &&clazz, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetStaticByteField, std::forward<Class>(clazz), fieldId);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetStaticCharField(JNIEnv *env, Class &&clazz, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetStaticCharField, std::forward<Class>(clazz), fieldId);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetStaticShortField(JNIEnv *env, Class &&clazz, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetStaticShortField, std::forward<Class>(clazz), fieldId);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetStaticIntField(JNIEnv *env, Class &&clazz, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetStaticIntField, std::forward<Class>(clazz), fieldId);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetStaticLongField(JNIEnv *env, Class &&clazz, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetStaticLongField, std::forward<Class>(clazz), fieldId);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetStaticFloatField(JNIEnv *env, Class &&clazz, jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetStaticFloatField, std::forward<Class>(clazz), fieldId);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetStaticDoubleField(JNIEnv *env, Class &&clazz,
                                                          jfieldID fieldId) {
        return JNI_SafeInvoke(env, &JNIEnv::GetStaticDoubleField, std::forward<Class>(clazz), fieldId);
    }

// setters

    template <ScopeOrClass Class, ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_SetStaticObjectField(JNIEnv *env, Class &&clazz, jfieldID fieldId,
                                                          const Object &value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetStaticObjectField, std::forward<Class>(clazz), fieldId,
                              value);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_SetStaticBooleanField(JNIEnv *env, Class &&clazz, jfieldID fieldId,
                                                           jboolean value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetStaticBooleanField, std::forward<Class>(clazz), fieldId,
                              value);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_SetStaticByteField(JNIEnv *env, Class &&clazz, jfieldID fieldId,
                                                        jbyte value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetStaticByteField, std::forward<Class>(clazz), fieldId,
                              value);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_SetStaticCharField(JNIEnv *env, Class &&clazz, jfieldID fieldId,
                                                        jchar value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetStaticCharField, std::forward<Class>(clazz), fieldId,
                              value);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_SetStaticShortField(JNIEnv *env, Class &&clazz, jfieldID fieldId,
                                                         jshort value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetStaticShortField, std::forward<Class>(clazz), fieldId,
                              value);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_SetStaticIntField(JNIEnv *env, Class &&clazz, jfieldID fieldId,
                                                       jint value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetStaticIntField, std::forward<Class>(clazz), fieldId,
                              value);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_SetStaticLongField(JNIEnv *env, Class &&clazz, jfieldID fieldId,
                                                        jlong value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetStaticLongField, std::forward<Class>(clazz), fieldId,
                              value);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_SetStaticFloatField(JNIEnv *env, Class &&clazz, jfieldID fieldId,
                                                         jfloat value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetStaticFloatField, std::forward<Class>(clazz), fieldId,
                              value);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_SetStaticDoubleField(JNIEnv *env, Class &&clazz, jfieldID fieldId,
                                                          jdouble value) {
        return JNI_SafeInvoke(env, &JNIEnv::SetStaticDoubleField, std::forward<Class>(clazz), fieldId,
                              value);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_ToReflectedMethod(JNIEnv *env, Class &&clazz, jmethodID method,
                                                       jboolean isStatic = JNI_FALSE) {
        return JNI_SafeInvoke(env, &JNIEnv::ToReflectedMethod, std::forward<Class>(clazz), method,
                              isStatic);
    }

// functions to method

// virtual methods

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetMethodID(JNIEnv *env, Class &&clazz, std::string_view name,
                                                 std::string_view sig) {
        return JNI_SafeInvoke(env, &JNIEnv::GetMethodID, std::forward<Class>(clazz), name, sig);
    }

    template <ScopeOrObject Object, typename... Args>
    [[maybe_unused]] inline auto JNI_CallVoidMethod(JNIEnv *env, Object &&obj, jmethodID method,
                                                    Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallVoidMethod, std::forward<Object>(obj), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, typename... Args>
    [[maybe_unused]] inline auto JNI_CallObjectMethod(JNIEnv *env, Object &&obj, jmethodID method,
                                                      Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallObjectMethod, std::forward<Object>(obj), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, typename... Args>
    [[maybe_unused]] inline auto JNI_CallBooleanMethod(JNIEnv *env, Object &&obj, jmethodID method,
                                                       Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallBooleanMethod, std::forward<Object>(obj), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, typename... Args>
    [[maybe_unused]] inline auto JNI_CallByteMethod(JNIEnv *env, Object &&obj, jmethodID method,
                                                    Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallByteMethod, std::forward<Object>(obj), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, typename... Args>
    [[maybe_unused]] inline auto JNI_CallCharMethod(JNIEnv *env, Object &&obj, jmethodID method,
                                                    Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallCharMethod, std::forward<Object>(obj), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, typename... Args>
    [[maybe_unused]] inline auto JNI_CallShortMethod(JNIEnv *env, Object &&obj, jmethodID method,
                                                     Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallShortMethod, std::forward<Object>(obj), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, typename... Args>
    [[maybe_unused]] inline auto JNI_CallIntMethod(JNIEnv *env, Object &&obj, jmethodID method,
                                                   Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallIntMethod, std::forward<Object>(obj), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, typename... Args>
    [[maybe_unused]] inline auto JNI_CallLongMethod(JNIEnv *env, Object &&obj, jmethodID method,
                                                    Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallLongMethod, std::forward<Object>(obj), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, typename... Args>
    [[maybe_unused]] inline auto JNI_CallFloatMethod(JNIEnv *env, Object &&obj, jmethodID method,
                                                     Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallFloatMethod, std::forward<Object>(obj), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, typename... Args>
    [[maybe_unused]] inline auto JNI_CallDoubleMethod(JNIEnv *env, Object &&obj, jmethodID method,
                                                      Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallDoubleMethod, std::forward<Object>(obj), method,
                              std::forward<Args>(args)...);
    }

// static methods

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_GetStaticMethodID(JNIEnv *env, Class &&clazz,
                                                       std::string_view name, std::string_view sig) {
        return JNI_SafeInvoke(env, &JNIEnv::GetStaticMethodID, std::forward<Class>(clazz), name, sig);
    }

    template <ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallStaticVoidMethod(JNIEnv *env, Class &&clazz, jmethodID method,
                                                          Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallStaticVoidMethod, std::forward<Class>(clazz), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallStaticObjectMethod(JNIEnv *env, Class &&clazz,
                                                            jmethodID method, Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallStaticObjectMethod, std::forward<Class>(clazz), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallStaticBooleanMethod(JNIEnv *env, Class &&clazz,
                                                             jmethodID method, Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallStaticBooleanMethod, std::forward<Class>(clazz), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallStaticByteMethod(JNIEnv *env, Class &&clazz, jmethodID method,
                                                          Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallStaticByteMethod, std::forward<Class>(clazz), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallStaticCharMethod(JNIEnv *env, Class &&clazz, jmethodID method,
                                                          Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallStaticCharMethod, std::forward<Class>(clazz), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallStaticShortMethod(JNIEnv *env, Class &&clazz, jmethodID method,
                                                           Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallStaticShortMethod, std::forward<Class>(clazz), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallStaticIntMethod(JNIEnv *env, Class &&clazz, jmethodID method,
                                                         Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallStaticIntMethod, std::forward<Class>(clazz), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallStaticLongMethod(JNIEnv *env, Class &&clazz, jmethodID method,
                                                          Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallStaticLongMethod, std::forward<Class>(clazz), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallStaticFloatMethod(JNIEnv *env, Class &&clazz, jmethodID method,
                                                           Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallStaticFloatMethod, std::forward<Class>(clazz), method,
                              std::forward<Args>(args)...);
    }

    template <ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallStaticDoubleMethod(JNIEnv *env, Class &&clazz,
                                                            jmethodID method, Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallStaticDoubleMethod, std::forward<Class>(clazz), method,
                              std::forward<Args>(args)...);
    }

// non-vritual methods

    template <ScopeOrObject Object, ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallCallNonvirtualVoidMethod(JNIEnv *env, Object &&obj,
                                                                  Class &&clazz, jmethodID method,
                                                                  Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallNonvirtualVoidMethod, std::forward<Object>(obj),
                              std::forward<Class>(clazz), method, std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallCallNonvirtualObjectMethod(JNIEnv *env, Object &&obj,
                                                                    Class &&clazz, jmethodID method,
                                                                    Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallNonvirtualObjectMethod, std::forward<Object>(obj),
                              std::forward<Class>(clazz), method, std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallCallNonvirtualBooleanMethod(JNIEnv *env, Object &&obj,
                                                                     Class &&clazz, jmethodID method,
                                                                     Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallNonvirtualBooleanMethod, std::forward<Object>(obj),
                              std::forward<Class>(clazz), method, std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallCallNonvirtualByteMethod(JNIEnv *env, Object &&obj,
                                                                  Class &&clazz, jmethodID method,
                                                                  Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallNonvirtualByteMethod, std::forward<Object>(obj),
                              std::forward<Class>(clazz), method, std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallCallNonvirtualCharMethod(JNIEnv *env, Object &&obj,
                                                                  Class &&clazz, jmethodID method,
                                                                  Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallNonvirtualCharMethod, std::forward<Object>(obj),
                              std::forward<Class>(clazz), method, std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallCallNonvirtualShortMethod(JNIEnv *env, Object &&obj,
                                                                   Class &&clazz, jmethodID method,
                                                                   Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallNonvirtualShortMethod, std::forward<Object>(obj),
                              std::forward<Class>(clazz), method, std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallCallNonvirtualIntMethod(JNIEnv *env, Object &&obj,
                                                                 Class &&clazz, jmethodID method,
                                                                 Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallNonvirtualIntMethod, std::forward<Object>(obj),
                              std::forward<Class>(clazz), method, std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallCallNonvirtualLongMethod(JNIEnv *env, Object &&obj,
                                                                  Class &&clazz, jmethodID method,
                                                                  Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallNonvirtualLongMethod, std::forward<Object>(obj),
                              std::forward<Class>(clazz), method, std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallCallNonvirtualFloatMethod(JNIEnv *env, Object &&obj,
                                                                   Class &&clazz, jmethodID method,
                                                                   Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallNonvirtualFloatMethod, std::forward<Object>(obj),
                              std::forward<Class>(clazz), method, std::forward<Args>(args)...);
    }

    template <ScopeOrObject Object, ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_CallCallNonvirtualDoubleMethod(JNIEnv *env, Object &&obj,
                                                                    Class &&clazz, jmethodID method,
                                                                    Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::CallNonvirtualDoubleMethod, std::forward<Object>(obj),
                              std::forward<Class>(clazz), method, std::forward<Args>(args)...);
    }

    template <ScopeOrClass Class, typename... Args>
    [[maybe_unused]] inline auto JNI_NewObject(JNIEnv *env, Class &&clazz, jmethodID method,
                                               Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::NewObject, std::forward<Class>(clazz), method,
                              std::forward<Args>(args)...);
    }

    template <typename... Args>
    [[maybe_unused]] inline auto JNI_NewDirectByteBuffer(JNIEnv *env, Args &&...args) {
        return JNI_SafeInvoke(env, &JNIEnv::NewDirectByteBuffer, std::forward<Args>(args)...);
    }

    template <ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_RegisterNatives(JNIEnv *env, Class &&clazz,
                                                     const JNINativeMethod *methods, jint size) {
        return JNI_SafeInvoke(env, &JNIEnv::RegisterNatives, std::forward<Class>(clazz), methods, size);
    }

    template <ScopeOrObject Object, ScopeOrClass Class>
    [[maybe_unused]] inline auto JNI_IsInstanceOf(JNIEnv *env, Object &&obj, Class &&clazz) {
        return JNI_SafeInvoke(env, &JNIEnv::IsInstanceOf, std::forward<Object>(obj),
                              std::forward<Class>(clazz));
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_NewGlobalRef(JNIEnv *env, Object &&x) {
        return (decltype(UnwrapScope(std::forward<Object>(x))))env->NewGlobalRef(
                UnwrapScope(std::forward<Object>(x)));
    }

    template <typename U, typename T>
    [[maybe_unused]] inline auto JNI_Cast(ScopedLocalRef<T> &&x) requires(
            std::is_convertible_v<T, _jobject *>) {
                    return ScopedLocalRef<U>(std::move(x));
            }

    [[maybe_unused]] inline auto JNI_NewDirectByteBuffer(JNIEnv *env, void *address, jlong capacity) {
        return JNI_SafeInvoke(env, &JNIEnv::NewDirectByteBuffer, address, capacity);
    }

    template <JArray T>
    struct JArrayUnderlyingTypeHelper;

    template <>
    struct JArrayUnderlyingTypeHelper<jobjectArray> {
        using Type = ScopedLocalRef<jobject>;
    };

    template <>
    struct JArrayUnderlyingTypeHelper<jbooleanArray> {
        using Type = jboolean;
    };

    template <>
    struct JArrayUnderlyingTypeHelper<jbyteArray> {
        using Type = jbyte;
    };

    template <>
    struct JArrayUnderlyingTypeHelper<jcharArray> {
        using Type = jchar;
    };

    template <>
    struct JArrayUnderlyingTypeHelper<jshortArray> {
        using Type = jshort;
    };

    template <>
    struct JArrayUnderlyingTypeHelper<jintArray> {
        using Type = jint;
    };

    template <>
    struct JArrayUnderlyingTypeHelper<jlongArray> {
        using Type = jlong;
    };

    template <>
    struct JArrayUnderlyingTypeHelper<jfloatArray> {
        using Type = jfloat;
    };

    template <>
    struct JArrayUnderlyingTypeHelper<jdoubleArray> {
        using Type = jdouble;
    };

    template <JArray T>
    using JArrayUnderlyingType = typename JArrayUnderlyingTypeHelper<T>::Type;

    template <JArray T>
    class ScopedLocalRef<T> {
        ScopedLocalRef(JNIEnv *env, T local_ref, size_t size, JArrayUnderlyingType<T> *elements,
                       bool modified) noexcept
                : env_(env), local_ref_(local_ref), size_(size), elements_(elements), modified_(modified) {}

    public:
        class Iterator {
            friend class ScopedLocalRef<T>;
            Iterator(JArrayUnderlyingType<T> *e) : e_(e) {}
            JArrayUnderlyingType<T> *e_;

        public:
            auto &operator*() { return *e_; }
            auto *operator->() { return e_; }
            Iterator &operator++() { return ++e_, *this; }
            Iterator &operator--() { return --e_, *this; }
            Iterator operator++(int) { return Iterator(e_++); }
            Iterator operator--(int) { return Iterator(e_--); }
            bool operator==(const Iterator &other) const { return other.e_ == e_; }
            bool operator!=(const Iterator &other) const { return other.e_ != e_; }
        };

        class ConstIterator {
            friend class ScopedLocalRef<T>;
            ConstIterator(const JArrayUnderlyingType<T> *e) : e_(e) {}
            const JArrayUnderlyingType<T> *e_;

        public:
            const auto &operator*() { return *e_; }
            const auto *operator->() { return e_; }
            ConstIterator &operator++() { return ++e_, *this; }
            ConstIterator &operator--() { return --e_, *this; }
            ConstIterator operator++(int) { return ConstIterator(e_++); }
            ConstIterator operator--(int) { return ConstIterator(e_--); }
            bool operator==(const ConstIterator &other) const { return other.e_ == e_; }
            bool operator!=(const ConstIterator &other) const { return other.e_ != e_; }
        };

        auto begin() {
            modified_ = true;
            return Iterator(elements_);
        }

        auto end() {
            modified_ = true;
            return Iterator(elements_ + size_);
        }

        auto begin() const { return ConstIterator(elements_); }

        auto end() const { return ConstIterator(elements_ + size_); }

        auto cbegin() const { return ConstIterator(elements_); }

        auto cend() const { return ConstIterator(elements_ + size_); }

        using BaseType [[maybe_unused]] = T;

        ScopedLocalRef(JNIEnv *env, T local_ref) noexcept : env_(env), local_ref_(nullptr) {
            reset(local_ref);
        }

        ScopedLocalRef(ScopedLocalRef &&s) noexcept
                : ScopedLocalRef(s.env_, s.local_ref_, s.size_, s.elements_, s.modified_) {
                s.local_ref_ = nullptr;
                s.size_ = 0;
                s.elements_ = nullptr;
                s.modified_ = false;
        }

        template <JObject U>
        ScopedLocalRef(ScopedLocalRef<U> &&s) noexcept : ScopedLocalRef(s.env_, (T)s.release()) {}

        explicit ScopedLocalRef(JNIEnv *env) noexcept : ScopedLocalRef(env, T{nullptr}) {}

        ~ScopedLocalRef() { env_->DeleteLocalRef(release()); }

        void reset(T ptr = nullptr) {
            if (ptr != local_ref_) {
                if (local_ref_ != nullptr) {
                    ReleaseElements(modified_ ? 0 : JNI_ABORT);
                    env_->DeleteLocalRef(local_ref_);
                    if constexpr (std::is_same_v<T, jobjectArray>) {
                        for (size_t i = 0; i < size_; ++i) {
                            elements_[i].~ScopedLocalRef<jobject>();
                        }
                        operator delete[](elements_);
                    }
                    elements_ = nullptr;
                }
                local_ref_ = ptr;
                size_ = local_ref_ ? env_->GetArrayLength(local_ref_) : 0;
                if (!local_ref_) return;
                if constexpr (std::is_same_v<T, jobjectArray>) {
                    elements_ = static_cast<ScopedLocalRef<jobject> *>(operator new[](
                            sizeof(ScopedLocalRef<jobject>) * size_));
                    for (size_t i = 0; i < size_; ++i) {
                        new (&elements_[i]) ScopedLocalRef<jobject>(
                                JNI_SafeInvoke(env_, &JNIEnv::GetObjectArrayElement, local_ref_, i));
                    }
                } else if constexpr (std::is_same_v<T, jbooleanArray>) {
                    elements_ = env_->GetBooleanArrayElements(local_ref_, nullptr);
                } else if constexpr (std::is_same_v<T, jbyteArray>) {
                    elements_ = env_->GetByteArrayElements(local_ref_, nullptr);
                } else if constexpr (std::is_same_v<T, jcharArray>) {
                    elements_ = env_->GetCharArrayElements(local_ref_, nullptr);
                } else if constexpr (std::is_same_v<T, jshortArray>) {
                    elements_ = env_->GetShortArrayElements(local_ref_, nullptr);
                } else if constexpr (std::is_same_v<T, jintArray>) {
                    elements_ = env_->GetIntArrayElements(local_ref_, nullptr);
                } else if constexpr (std::is_same_v<T, jlongArray>) {
                    elements_ = env_->GetLongArrayElements(local_ref_, nullptr);
                } else if constexpr (std::is_same_v<T, jfloatArray>) {
                    elements_ = env_->GetFloatArrayElements(local_ref_, nullptr);
                } else if constexpr (std::is_same_v<T, jdoubleArray>) {
                    elements_ = env_->GetDoubleArrayElements(local_ref_, nullptr);
                }
            }
        }

        [[nodiscard]] T release() {
            T localRef = local_ref_;
            size_ = 0;
            local_ref_ = nullptr;
            ReleaseElements(modified_ ? 0 : JNI_ABORT);
            if constexpr (std::is_same_v<T, jobjectArray>) {
                for (size_t i = 0; i < size_; ++i) {
                    elements_[i].~ScopedLocalRef<jobject>();
                }
                operator delete[](elements_);
            }
            elements_ = nullptr;
            return localRef;
        }

        T get() const { return local_ref_; }

        explicit operator T() const { return local_ref_; }

        JArrayUnderlyingType<T> &operator[](size_t index) {
            modified_ = true;
            return elements_[index];
        }

        const JArrayUnderlyingType<T> &operator[](size_t index) const { return elements_[index]; }

        void commit() {
            ReleaseElements(JNI_COMMIT);
            modified_ = false;
        }

        // We do not expose an empty constructor as it can easily lead to errors
        // using common idioms, e.g.:
        //   ScopedLocalRef<...> ref;
        //   ref.reset(...);
        // Move assignment operator.
        ScopedLocalRef &operator=(ScopedLocalRef &&s) noexcept {
            env_ = s.env_;
            local_ref_ = s.local_ref_;
            size_ = s.size_;
            elements_ = s.elements_;
            modified_ = s.modified_;
            s.elements_ = nullptr;
            s.size_ = 0;
            s.modified_ = false;
            s.local_ref_ = nullptr;
            return *this;
        }

        size_t size() const { return size_; }

        operator bool() const { return local_ref_; }

        template <JObject U>
        friend class ScopedLocalRef;

        friend class JUTFString;

    private:
        void ReleaseElements(jint mode) {
            if (!local_ref_ || !elements_) return;
            if constexpr (std::is_same_v<T, jobjectArray>) {
                for (size_t i = 0; i < size_; ++i) {
                    JNI_SafeInvoke(env_, &JNIEnv::SetObjectArrayElement, local_ref_, i, elements_[i]);
                }
            } else if constexpr (std::is_same_v<T, jbooleanArray>) {
                env_->ReleaseBooleanArrayElements(local_ref_, elements_, mode);
            } else if constexpr (std::is_same_v<T, jbyteArray>) {
                env_->ReleaseByteArrayElements(local_ref_, elements_, mode);
            } else if constexpr (std::is_same_v<T, jcharArray>) {
                env_->ReleaseCharArrayElements(local_ref_, elements_, mode);
            } else if constexpr (std::is_same_v<T, jshortArray>) {
                env_->ReleaseShortArrayElements(local_ref_, elements_, mode);
            } else if constexpr (std::is_same_v<T, jintArray>) {
                env_->ReleaseIntArrayElements(local_ref_, elements_, mode);
            } else if constexpr (std::is_same_v<T, jlongArray>) {
                env_->ReleaseLongArrayElements(local_ref_, elements_, mode);
            } else if constexpr (std::is_same_v<T, jfloatArray>) {
                env_->ReleaseFloatArrayElements(local_ref_, elements_, mode);
            } else if constexpr (std::is_same_v<T, jdoubleArray>) {
                env_->ReleaseDoubleArrayElements(local_ref_, elements_, mode);
            }
        }

        JNIEnv *env_;
        T local_ref_;
        size_t size_;
        JArrayUnderlyingType<T> *elements_{nullptr};
        bool modified_ = false;
        DISALLOW_COPY_AND_ASSIGN(ScopedLocalRef);
    };

// functions to array

    template <ScopeOrRaw<jarray> Array>
    [[maybe_unused]] inline auto JNI_GetArrayLength(JNIEnv *env, const Array &array) {
        return JNI_SafeInvoke(env, &JNIEnv::GetArrayLength, array);
    }

// newers

    template <ScopeOrClass Class, ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_NewObjectArray(JNIEnv *env, jsize len, Class &&clazz,
                                                    const Object &init) {
        return JNI_SafeInvoke(env, &JNIEnv::NewObjectArray, len, std::forward<Class>(clazz), init);
    }

    [[maybe_unused]] inline auto JNI_NewBooleanArray(JNIEnv *env, jsize len) {
        return JNI_SafeInvoke(env, &JNIEnv::NewBooleanArray, len);
    }

    [[maybe_unused]] inline auto JNI_NewByteArray(JNIEnv *env, jsize len) {
        return JNI_SafeInvoke(env, &JNIEnv::NewByteArray, len);
    }

    [[maybe_unused]] inline auto JNI_NewCharArray(JNIEnv *env, jsize len) {
        return JNI_SafeInvoke(env, &JNIEnv::NewCharArray, len);
    }

    [[maybe_unused]] inline auto JNI_NewShortArray(JNIEnv *env, jsize len) {
        return JNI_SafeInvoke(env, &JNIEnv::NewShortArray, len);
    }

    [[maybe_unused]] inline auto JNI_NewIntArray(JNIEnv *env, jsize len) {
        return JNI_SafeInvoke(env, &JNIEnv::NewIntArray, len);
    }

    [[maybe_unused]] inline auto JNI_NewLongArray(JNIEnv *env, jsize len) {
        return JNI_SafeInvoke(env, &JNIEnv::NewLongArray, len);
    }

    [[maybe_unused]] inline auto JNI_NewFloatArray(JNIEnv *env, jsize len) {
        return JNI_SafeInvoke(env, &JNIEnv::NewFloatArray, len);
    }

    [[maybe_unused]] inline auto JNI_NewDoubleArray(JNIEnv *env, jsize len) {
        return JNI_SafeInvoke(env, &JNIEnv::NewDoubleArray, len);
    }

    template <ScopeOrObject Object>
    [[maybe_unused]] inline auto JNI_GetObjectFieldOf(JNIEnv *env, Object &&object,
                                                      std::string_view field_name,
                                                      std::string_view field_class) {
        auto &&o = std::forward<Object>(object);
        return JNI_GetObjectField(
                env, o, JNI_GetFieldID(env, JNI_GetObjectClass(env, o), field_name, field_class));
    }

}  // namespace lsplant

#undef DISALLOW_COPY_AND_ASSIGN

#pragma clang diagnostic pop
