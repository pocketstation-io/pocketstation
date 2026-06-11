# Build the PocketStation HAL plugin .driver bundle.
# Requirements: Xcode command-line tools, Developer ID Application certificate.
#
# Usage:
#   make              — debug build
#   make release      — release build (code-signed, ready for pkg)
#   make install      — install to /Library/Audio/Plug-Ins/HAL/ (requires sudo)
#   make uninstall    — remove installed plugin

PLUGIN_NAME     = PocketStationLoopback
BUNDLE_ID       = io.pocketstation.loopback
BUILD_DIR       = build
RELEASE_DIR     = $(BUILD_DIR)/Release
BUNDLE_PATH     = $(RELEASE_DIR)/$(PLUGIN_NAME).driver

# Set to your Developer ID Application identity for notarization
SIGN_IDENTITY   ?= -

CFLAGS          = -std=c++17 -O2 -Wall -Wextra \
                  -framework CoreAudio -framework CoreFoundation \
                  -dynamiclib -install_name $(BUNDLE_PATH)/Contents/MacOS/$(PLUGIN_NAME)

SRCS            = Plugin.cpp
HEADERS         = SharedRing.h bridge.h

.PHONY: all release install uninstall clean

all: $(BUNDLE_PATH)

$(BUILD_DIR)/$(PLUGIN_NAME).dylib: $(SRCS) $(HEADERS)
	@mkdir -p $(BUILD_DIR)
	clang++ $(CFLAGS) -o $@ $(SRCS)

$(BUNDLE_PATH): $(BUILD_DIR)/$(PLUGIN_NAME).dylib
	@mkdir -p $(BUNDLE_PATH)/Contents/MacOS
	cp $< $(BUNDLE_PATH)/Contents/MacOS/$(PLUGIN_NAME)
	cp Info.plist $(BUNDLE_PATH)/Contents/Info.plist

release: $(BUNDLE_PATH)
	codesign --force --sign "$(SIGN_IDENTITY)" \
	         --options runtime --timestamp \
	         $(BUNDLE_PATH)

install: $(BUNDLE_PATH)
	sudo cp -R $(BUNDLE_PATH) /Library/Audio/Plug-Ins/HAL/
	sudo launchctl kickstart -k system/com.apple.audio.coreaudiod

uninstall:
	sudo rm -rf /Library/Audio/Plug-Ins/HAL/$(PLUGIN_NAME).driver
	sudo launchctl kickstart -k system/com.apple.audio.coreaudiod

clean:
	rm -rf $(BUILD_DIR)
