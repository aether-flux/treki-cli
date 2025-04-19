#!/bin/sh

BINARY_NAME="treki-cli"
TARGET_PATH="/usr/local/bin/treki"
RELEASE_URL="https://github.com/aether-flux/treki-cli/releases/download/v1.0.0/treki-cli"

echo "🚀 Installing $BINARY_NAME..."

# Download the binary from GitHub Releases
if command -v curl >/dev/null 2>&1; then
    curl -L "$RELEASE_URL" -o "$BINARY_NAME"
elif command -v wget >/dev/null 2>&1; then
    wget "$RELEASE_URL" -O "$BINARY_NAME"
else
    echo "❌ Error: curl or wget required to download the binary."
    exit 1
fi

# Make it executable
chmod +x "$BINARY_NAME"

# Move it to /usr/local/bin as 'treki'
if command -v sudo >/dev/null 2>&1; then
    sudo mv "$BINARY_NAME" "$TARGET_PATH"
else
    echo "⚠️ 'sudo' not found. Attempting to install without it..."
    mv "$BINARY_NAME" "$TARGET_PATH" || {
        echo "❌ Failed to move binary to $TARGET_PATH. Try running as root."
        exit 1
    }
fi

echo "✅ treki installed to $TARGET_PATH"
echo "🎉 You can now run 'treki' from anywhere!"

