#!/bin/sh

BINARY_NAME="treki-cli"
TARGET_PATH="/usr/local/bin/$BINARY_NAME"
RELEASE_BINARY="target/release/$BINARY_NAME"

echo "🚀 Installing $BINARY_NAME..."

# Check if the release binary exists
if [ ! -f "$RELEASE_BINARY" ]; then
    echo "❌ Error: $RELEASE_BINARY not found. Please build the project first with 'cargo build --release'."
    exit 1
fi

# Copy the binary to /usr/local/bin
if command -v sudo >/dev/null 2>&1; then
    sudo cp "$RELEASE_BINARY" "$TARGET_PATH"
    sudo chmod +x "$TARGET_PATH"
else
    echo "⚠️ 'sudo' not found. Attempting to install without it..."
    cp "$RELEASE_BINARY" "$TARGET_PATH" || {
        echo "❌ Failed to copy binary to $TARGET_PATH. Try running as root."
        exit 1
    }
    chmod +x "$TARGET_PATH"
fi

echo "✅ $BINARY_NAME installed to $TARGET_PATH"
echo "🎉 You can now run '$BINARY_NAME' from anywhere!"

