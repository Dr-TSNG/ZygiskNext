plugins {
    alias(libs.plugins.agp.lib)
    alias(libs.plugins.rust.android)
}

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
    val isDebug = gradle.startParameter.taskNames.any { it.toLowerCase().contains("debug") }
    profile = if (isDebug) "debug" else "release"
    exec = { spec, _ ->
        spec.environment("ANDROID_NDK_HOME", android.ndkDirectory.path)
        spec.environment("MIN_KSU_VERSION", minKsuVersion)
        spec.environment("MAX_KSU_VERSION", maxKsuVersion)
        spec.environment("MIN_MAGISK_VERSION", minMagiskVersion)
    }
}

afterEvaluate {
    task<Task>("buildAndStrip") {
        dependsOn(":zygiskd:cargoBuild")
        doLast {
            val dir = File(buildDir, "rustJniLibs/android")
            val prebuilt = File(android.ndkDirectory, "toolchains/llvm/prebuilt").listFiles()!!.first()
            val binDir = File(prebuilt, "bin")
            println("binDir $binDir")
            val suffix = if (prebuilt.name.contains("windows")) ".exe" else ""
            val strip = File(binDir, "llvm-strip$suffix")
            val objcopy = File(binDir, "llvm-objcopy$suffix")
            dir.listFiles()!!.forEach {
                if (!it.isDirectory) return@forEach
                exec {
                    workingDir = it
                    commandLine(objcopy, "--only-keep-debug", "zygiskd", "zygiskd.debug")
                }
                exec {
                    workingDir = it
                    commandLine(strip, "--strip-all", "zygiskd")
                }
                exec {
                    workingDir = it
                    commandLine(objcopy, "--add-gnu-debuglink", "zygiskd.debug", "zygiskd")
                }
            }
        }
    }
}