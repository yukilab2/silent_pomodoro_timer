#!/bin/bash

# Build script for macOS .app bundle
# This creates a proper macOS application bundle that won't open a terminal window

set -e

APP_NAME="Pomodoro Timer"
BUNDLE_NAME="PomodoroTimer.app"
BINARY_NAME="pomodoro_timer"
BUILD_DIR="target/release"
APP_DIR="target/${BUNDLE_NAME}"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

echo "Building release binary..."
cargo build --release

echo "Creating .app bundle structure..."
# Remove existing bundle if it exists
rm -rf "${APP_DIR}"

# Create directory structure
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Copy binary
cp "${BUILD_DIR}/${BINARY_NAME}" "${MACOS_DIR}/${BINARY_NAME}"

# Make binary executable
chmod +x "${MACOS_DIR}/${BINARY_NAME}"

# Create Info.plist
cat > "${CONTENTS_DIR}/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${BINARY_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>com.pomodoro.timer</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSUIElement</key>
    <false/>
</dict>
</plist>
EOF

echo "✅ .app bundle created at: ${APP_DIR}"
echo ""
echo "To install, copy the bundle to Applications:"
echo "  cp -r ${APP_DIR} /Applications/"
echo ""
echo "Or open it in Finder:"
echo "  open ${APP_DIR}"

