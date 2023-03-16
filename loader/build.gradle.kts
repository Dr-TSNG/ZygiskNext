import java.nio.file.Paths
import org.gradle.internal.os.OperatingSystem

plugins {
    id("com.android.library")
}

fun Project.findInPath(executable: String, property: String): String? {
    val pathEnv = System.getenv("PATH")
    return pathEnv.split(File.pathSeparator).map { folder ->
        Paths.get("${folder}${File.separator}${executable}${if (OperatingSystem.current().isWindows) ".exe" else ""}")
            .toFile()
    }.firstOrNull { path ->
        path.exists()
    }?.absolutePath ?: properties.getOrDefault(property, null) as? String?
}

val ccachePatch by lazy {
    project.findInPath("ccache", "ccache.path")?.also {
        println("loader: Use ccache: $it")
    }
}

android {
    buildFeatures {
        androidResources = false
        buildConfig = false
        prefab = true
    }

    externalNativeBuild.ndkBuild {
        path("src/Android.mk")
    }

    defaultConfig {
        externalNativeBuild {
            ndkBuild {
                ccachePatch?.let {
                    arguments += "NDK_CCACHE=$it"
                }
            }
        }
    }
}

dependencies {
    implementation("dev.rikka.ndk.thirdparty:cxx:1.2.0")
}
