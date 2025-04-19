#!/bin/sh

CLI_NAME="treki"
BINARY_NAME="treki-cli"
INSTALL_PATH="/usr/local/bin/$CLI_NAME"
RELEASE_URL="https://github.com/aether-flux/treki-cli/releases/download/v1.0.0/$BINARY_NAME"

echo "🚀 Installing $CLI_NAME..."

# Download the binary
curl -L "$RELEASE_URL" -o "$BINARY_NAME"

# Make it executable
chmod +x "$BINARY_NAME"

# Move to /usr/local/bin as 'treki'
if command -v sudo >/dev/null 2>&1; then
    sudo mv "$BINARY_NAME" "$INSTALL_PATH"
else
    echo "⚠️ 'sudo' not found. Attempting to install without it..."
    mv "$BINARY_NAME" "$INSTALL_PATH" || {
        echo "❌ Failed to move binary to $INSTALL_PATH. Try running as root."
        exit 1
    }
fi

echo "✅ $CLI_NAME installed to $INSTALL_PATH"
echo "🎉 You can now run '$CLI_NAME' from anywhere!"

