import java.nio.file.Paths
import org.gradle.internal.os.OperatingSystem

plugins {
    alias(libs.plugins.agp.lib)
}

val verCode: Int by rootProject.extra
val verName: String by rootProject.extra
val commitHash: String by rootProject.extra

fun Project.findInPath(executable: String, property: String): String? {
    val pathEnv = System.getenv("PATH")
    return pathEnv.split(File.pathSeparator).map { folder ->
        Paths.get("${folder}${File.separator}${executable}${if (OperatingSystem.current().isWindows) ".exe" else ""}")
            .toFile()
    }.firstOrNull { path ->
        path.exists()
    }?.absolutePath ?: properties.getOrDefault(property, null) as? String?
}

val ccachePath by lazy {
    project.findInPath("ccache", "ccache.path")?.also {
        println("loader: Use ccache: $it")
    }
}

val defaultCFlags = arrayOf(
    "-Wall", "-Wextra",
    "-fno-rtti", "-fno-exceptions",
    "-fno-stack-protector", "-fomit-frame-pointer",
    "-Wno-builtin-macro-redefined", "-D__FILE__=__FILE_NAME__"
)

val releaseFlags = arrayOf(
    "-Oz", "-flto",
    "-Wno-unused", "-Wno-unused-parameter",
    "-fvisibility=hidden", "-fvisibility-inlines-hidden",
    "-fno-unwind-tables", "-fno-asynchronous-unwind-tables",
    "-Wl,--exclude-libs,ALL", "-Wl,--gc-sections", "-Wl,--strip-all"
)

android {
    buildFeatures {
        androidResources = false
        buildConfig = false
        prefab = true
    }

    externalNativeBuild.cmake {
        path("src/CMakeLists.txt")
    }

    defaultConfig {
        externalNativeBuild.cmake {
            arguments += "-DANDROID_STL=none"
            arguments += "-DLSPLT_STANDALONE=ON"
            cFlags("-std=c18", *defaultCFlags)
            cppFlags("-std=c++20", *defaultCFlags)
            ccachePath?.let {
                arguments += "-DNDK_CCACHE=$it"
            }
        }
    }

    buildTypes {
        debug {
            externalNativeBuild.cmake {
                arguments += "-DZKSU_VERSION=$verName-$verCode-$commitHash-debug"
            }
        }
        release {
            externalNativeBuild.cmake {
                cFlags += releaseFlags
                cppFlags += releaseFlags
                arguments += "-DZKSU_VERSION=$verName-$verCode-$commitHash-release"
            }
        }
    }
}

dependencies {
    implementation("dev.rikka.ndk.thirdparty:cxx:1.2.0")
}
