plugins {
    id("com.android.library")
}

android {
    buildFeatures {
        prefab = true
    }

    externalNativeBuild.ndkBuild {
        path("src/Android.mk")
    }
}

dependencies {
    implementation("dev.rikka.ndk.thirdparty:cxx:1.2.0")
}
