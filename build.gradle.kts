import com.android.build.api.dsl.ApplicationExtension
import com.android.build.gradle.BaseExtension

plugins {
    id("com.android.application") apply false
    id("com.android.library") apply false
}

val verCode by extra(25207)
val verName by extra("25.2-1")
val androidMinSdkVersion by extra(29)
val androidTargetSdkVersion by extra(33)
val androidCompileSdkVersion by extra(33)
val androidBuildToolsVersion by extra("33.0.1")
val androidCompileNdkVersion by extra("25.1.8937393")
val androidSourceCompatibility by extra(JavaVersion.VERSION_11)
val androidTargetCompatibility by extra(JavaVersion.VERSION_11)

tasks.register("Delete", Delete::class) {
    delete(rootProject.buildDir)
}

fun Project.configureBaseExtension() {
    extensions.findByType(BaseExtension::class)?.run {
        compileSdkVersion(androidCompileSdkVersion)
        ndkVersion = androidCompileNdkVersion
        buildToolsVersion = androidBuildToolsVersion

        defaultConfig {
            minSdk = androidMinSdkVersion
            targetSdk = androidTargetSdkVersion
            versionCode = verCode
            versionName = verName
        }
    }

    extensions.findByType(ApplicationExtension::class)?.lint {
        abortOnError = true
        checkReleaseBuilds = false
    }
}

subprojects {
    plugins.withId("com.android.application") {
        configureBaseExtension()
    }
    plugins.withId("com.android.library") {
        configureBaseExtension()
    }
}
