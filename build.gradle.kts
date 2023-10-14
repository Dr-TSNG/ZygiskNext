import com.android.build.gradle.LibraryExtension
import java.io.ByteArrayOutputStream

plugins {
    id("com.android.application") apply false
    id("com.android.library") apply false
}

buildscript {
    repositories {
        maven("https://plugins.gradle.org/m2/")
    }
    dependencies {
        classpath("org.eclipse.jgit:org.eclipse.jgit:6.4.0.202211300538-r")
        classpath("org.mozilla.rust-android-gradle:plugin:0.9.3")
    }
}

fun String.execute(currentWorkingDir: File = file("./")): String {
    val byteOut = ByteArrayOutputStream()
    project.exec {
        workingDir = currentWorkingDir
        commandLine = split("\\s".toRegex())
        standardOutput = byteOut
    }
    return String(byteOut.toByteArray()).trim()
}

val gitCommitCount = "git rev-list HEAD --count".execute().toInt()
val gitCommitHash = "git rev-parse --verify --short HEAD".execute()

val moduleId by extra("zygisksu")
val moduleName by extra("Zygisk on KernelSU")
val verName by extra("v4-0.7.1")
val verCode by extra(gitCommitCount)
val commitHash by extra(gitCommitHash)
val minKsuVersion by extra(10940)
val minKsudVersion by extra(10942)
val maxKsuVersion by extra(20000)
val minMagiskVersion by extra(25208)

val androidMinSdkVersion by extra(29)
val androidTargetSdkVersion by extra(34)
val androidCompileSdkVersion by extra(34)
val androidBuildToolsVersion by extra("34.0.0")
val androidCompileNdkVersion by extra("26.0.10792818")
val androidSourceCompatibility by extra(JavaVersion.VERSION_11)
val androidTargetCompatibility by extra(JavaVersion.VERSION_11)

tasks.register("Delete", Delete::class) {
    delete(rootProject.buildDir)
}

fun Project.configureBaseExtension() {
    extensions.findByType(LibraryExtension::class)?.run {
        namespace = "icu.nullptr.zygisksu"
        compileSdk = androidCompileSdkVersion
        ndkVersion = androidCompileNdkVersion
        buildToolsVersion = androidBuildToolsVersion

        defaultConfig {
            minSdk = androidMinSdkVersion
            targetSdk = androidTargetSdkVersion
        }

        lint {
            abortOnError = true
        }
    }
}

subprojects {
    plugins.withId("com.android.library") {
        configureBaseExtension()
    }
}
