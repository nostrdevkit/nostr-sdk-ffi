buildscript {
    repositories {
        google()
        mavenCentral()
    }
    dependencies {
        classpath("com.android.tools.build:gradle:8.3.0")
    }
}

plugins {
    id("org.jetbrains.dokka") version "1.9.20" apply false
}
