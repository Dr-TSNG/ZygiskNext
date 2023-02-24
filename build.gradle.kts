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
val verName by extra("v4-0.5.0")
val verCode by extra(gitCommitCount)
val minKsuVersion by extra(10654)
val minKsudVersion by extra(10647)
val maxKsuVersion by extra(20000)
val minMagiskVersion by extra(25000)

val androidMinSdkVersion by extra(29)
val androidTargetSdkVersion by extra(33)
val androidCompileSdkVersion by extra(33)
val androidBuildToolsVersion by extra("33.0.2")
val androidCompileNdkVersion by extra("25.2.9519653")
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
