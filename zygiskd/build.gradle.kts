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
    targetDirectory = "build/intermediates/rust"
    exec = { spec, _ ->
        spec.environment("CARGO_TARGET_DIR", targetDirectory)
    }
}

androidComponents.onVariants { variant ->
    val variantCapped = variant.name.capitalize()
    task("build$variantCapped") {
        group = "zygiskd"
        cargo.targets?.forEach {
            dependsOn("cargoBuild${it.capitalize()}")
        }
    }

    task<Exec>("pushAndRun$variantCapped") {
        group = "zygiskd"
        dependsOn("build$variantCapped")
        doLast {
            commandLine("adb", "push", "target/")
        }
    }
}
