plugins {
    id("com.android.library")
    id("org.mozilla.rust-android-gradle.rust-android")
}

val verName: String by rootProject.extra
val verCode: Int by rootProject.extra
val minKsuVersion: Int by rootProject.extra
val maxKsuVersion: Int by rootProject.extra
val minMagiskVersion: Int by rootProject.extra

android.buildFeatures {
    androidResources = false
    buildConfig = false
}

cargo {
    module = "."
    libname = "zygiskd"
    targetIncludes = arrayOf("zygiskd")
    targets = listOf("arm64", "arm", "x86", "x86_64")
    targetDirectory = "build/intermediates/rust"
    val isDebug = gradle.startParameter.taskNames.any { it.lowercase().contains("debug") }
    profile = if (isDebug) "debug" else "release"
    exec = { spec, _ ->
        spec.environment("ANDROID_NDK_HOME", android.ndkDirectory.path)
        spec.environment("VERSION_CODE", verCode)
        spec.environment("VERSION_NAME", verName)
        spec.environment("MIN_KSU_VERSION", minKsuVersion)
        spec.environment("MAX_KSU_VERSION", maxKsuVersion)
        spec.environment("MIN_MAGISK_VERSION", minMagiskVersion)
    }
}
