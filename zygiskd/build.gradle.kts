plugins {
    id("com.android.library")
    id("org.mozilla.rust-android-gradle.rust-android")
}

val verName: String by rootProject.extra
val verCode: Int by rootProject.extra

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
    val isDebug = gradle.startParameter.taskNames.any { it.toLowerCase().contains("debug") }
    profile = if (isDebug) "debug" else "release"
    exec = { spec, _ ->
        spec.environment("VERSION_CODE", verCode)
        spec.environment("VERSION_NAME", verName)
    }
}
