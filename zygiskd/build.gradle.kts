plugins {
    id("com.android.library")
    id("org.mozilla.rust-android-gradle.rust-android")
}

cargo {
    module = "."
    libname = "zygiskd"
    targetIncludes = arrayOf("zygiskd")
    targets = listOf("arm64", "arm", "x86", "x86_64")
    val isDebug = gradle.startParameter.taskNames.any { it.toLowerCase().contains("debug") }
    profile = if (isDebug) "debug" else "release"
}

androidComponents.onVariants { variant ->
    val variantCapped = variant.name.capitalize()
    task("build$variantCapped") {
        group = "zygiskd"
        cargo.targets?.forEach {
            dependsOn("cargoBuild${it.capitalize()}")
        }
    }

    task("push$variantCapped") {
        group = "zygiskd"
        dependsOn("cargoBuildArm", "cargoBuildArm64")
        doLast {
            val moduleDir = "/data/adb/ksu/modules/zygisksu"
            exec { commandLine("adb", "push", "build/rustJniLibs/android/armeabi-v7a/zygiskd", "/data/local/tmp/zygiskd32") }
            exec { commandLine("adb", "push", "build/rustJniLibs/android/arm64-v8a/zygiskd", "/data/local/tmp/zygiskd64") }
            exec { commandLine("adb", "shell", "su", "-c", "mv /data/local/tmp/zygiskd32 $moduleDir/zygiskd32") }
            exec { commandLine("adb", "shell", "su", "-c", "mv /data/local/tmp/zygiskd64 $moduleDir/zygiskd64") }
            exec { commandLine("adb", "shell", "su", "-c", "ln -sf zygiskd64 $moduleDir/zygiskwd") }
        }
    }
}
