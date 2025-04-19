#!/bin/sh

BINARY_NAME="treki-cli"
CLI_NAME="treki"
TARGET_PATH="/usr/local/bin/$CLI_NAME"
LATEST_RELEASE_URL="https://github.com/aether-flux/treki-cli/releases/latest/download/v1.0.0/$BINARY_NAME"

echo "🚀 Installing $CLI_NAME..."

# Download the binary
curl -L "$LATEST_RELEASE_URL" -o "$BINARY_NAME" || {
    echo "❌ Failed to download binary."
    exit 1
}

chmod +x "$BINARY_NAME"

# Move it to /usr/local/bin
if command -v sudo >/dev/null 2>&1; then
    sudo mv "$BINARY_NAME" "$TARGET_PATH"
else
    echo "⚠️ 'sudo' not found. Attempting to move without it..."
    mv "$BINARY_NAME" "$TARGET_PATH" || {
        echo "❌ Failed to move binary to $TARGET_PATH. Try running as root."
        exit 1
    }
fi

echo "✅ $CLI_NAME installed to $TARGET_PATH"
echo "🎉 You can now run '$CLI_NAME' from anywhere!"

