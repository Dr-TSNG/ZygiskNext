plugins {
    id("com.android.library")
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
}

dependencies {
    implementation("dev.rikka.ndk.thirdparty:cxx:1.2.0")
    implementation("org.lsposed.lsplt:lsplt-standalone:1.1")
}
