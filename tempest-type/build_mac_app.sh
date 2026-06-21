#!/bin/bash

# Exit on error
set -e

APP_NAME="TempestType"
APP_DIR="$APP_NAME.app"
BIN_NAME="tempest-type"

echo "🔨 Building release binary..."
cargo build --release

echo "📁 Creating application bundle structure..."
rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

echo "📋 Generating Info.plist..."
cat <<EOF > "$APP_DIR/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$APP_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>com.tempest.type.stable</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>LSUIElement</key>
    <true/>
    <key>NSMicrophoneUsageDescription</key>
    <string>Tempest Type needs microphone access to dictate your speech.</string>
    <key>NSAppleEventsUsageDescription</key>
    <string>Tempest Type needs accessibility access to type text for you.</string>
</dict>
</plist>
EOF

echo "📦 Copying binary..."
cp "target/release/$BIN_NAME" "$APP_DIR/Contents/MacOS/$APP_NAME"

echo "🔐 Signing application bundle..."
codesign --force --deep --sign - "$APP_DIR"

echo "✅ Build complete! You can now run $APP_DIR"
echo "NOTE: When you run it for the first time, macOS should prompt you for permissions."
