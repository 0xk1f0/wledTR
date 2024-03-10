# Makefile

# Default target
all: android

# Target to init
init:
	@echo "Setting up..."
	npm install
	ANDROID_HOME="/opt/android-sdk" \
	NDK_HOME="/opt/android-sdk/ndk/25.2.9519653" \
	JAVA_HOME="/usr/lib/jvm/java-17-openjdk" \
	npm run tauri android init

# linux build
linux:
	@echo "Building..."
	NO_STRIP=true \
	npm run tauri build

# android build
android:
	@echo "Building..."
	ANDROID_HOME="/opt/android-sdk" \
	NDK_HOME="/opt/android-sdk/ndk/25.2.9519653" \
	JAVA_HOME="/usr/lib/jvm/java-17-openjdk" \
	npm run tauri android build

# Clean target
clean: 
	@echo "Cleaning..."
	rm -rf ./src-tauri/gen/android/
	rm -rf ./src-tauri/target/
	rm -rf build/
	rm -rf .svelte-kit/

.PHONY: setup android linux clean
