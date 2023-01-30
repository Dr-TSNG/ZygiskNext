plugins {
    id("com.android.library")
}

val verCode: Int by rootProject.extra
val verName: String by rootProject.extra

android.buildFeatures {
    androidResources = false
    buildConfig = false
}
