# ProGuard rules for Reality Portal Android App

# Keep Kotlin Serialization
-keepattributes *Annotation*, InnerClasses
-dontnote kotlinx.serialization.AnnotationsKt

-keepclassmembers class kotlinx.serialization.json.** {
    *** Companion;
}
-keepclasseswithmembers class kotlinx.serialization.json.** {
    kotlinx.serialization.KSerializer serializer(...);
}

-keep,includedescriptorclasses class three.two.bit.ppt.reality.**$$serializer { *; }
-keepclassmembers class three.two.bit.ppt.reality.** {
    *** Companion;
}
-keepclasseswithmembers class three.two.bit.ppt.reality.** {
    kotlinx.serialization.KSerializer serializer(...);
}

# Keep Ktor
-keep class io.ktor.** { *; }
-keepclassmembers class io.ktor.** { *; }

# Keep Coroutines
-keepclassmembernames class kotlinx.** {
    volatile <fields>;
}

# Keep data classes for serialization
-keep class three.two.bit.ppt.reality.models.** { *; }
