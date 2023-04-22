plugins {
    kotlin("multiplatform") version "1.8.20"
}

repositories {
    mavenCentral()
}

kotlin {
    // linuxX64("native") { // on Linux
    // macosX64("native") { // on x86_64 macOS
    // macosArm64("native") { // on Apple Silicon macOS
    mingwX64("native") { // on Windows
        binaries {
            sharedLib {
                baseName = "native" // on Linux and macOS
                // baseName = "libnative" // on Windows
            }
        }
    }
}

tasks.withType<Wrapper> {
    gradleVersion = "7.6"
    distributionType = Wrapper.DistributionType.BIN
}