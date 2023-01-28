plugins {
    id("com.android.application")
}

val moduleName = "Zygisk On KernelSU"
val moduleBaseId = "zygisksu"
val authors = "Nullptr"

val verCode: Int by rootProject.extra
val verName: String by rootProject.extra

android {
    namespace = "icu.nullptr.zygisksu"
}
