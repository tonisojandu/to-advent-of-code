plugins {
    id("org.jetbrains.kotlin.jvm") version "1.4.31"
}

repositories {
    mavenCentral()
    mavenLocal()
}

dependencies {
    implementation(platform("org.jetbrains.kotlin:kotlin-bom"))
    implementation("org.jetbrains.kotlin:kotlin-stdlib")

    testImplementation("org.assertj:assertj-core:3.21.0")
    testImplementation("org.junit.jupiter:junit-jupiter-api:5.8.1")

    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.8.1")

}

tasks.named("test", Test::class) {
    useJUnitPlatform()
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions {
        freeCompilerArgs = listOf("-Xjsr305=strict")
        jvmTarget = "11"
    }
}
