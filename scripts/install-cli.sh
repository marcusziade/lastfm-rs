#!/bin/bash

# Last.fm CLI Installation Script

set -e

echo "================================"
echo "Last.fm CLI Installation"
echo "================================"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed."
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if wrangler is installed (optional)
if ! command -v wrangler &> /dev/null; then
    echo "Warning: Wrangler CLI is not installed."
    echo "Worker management commands will not work without it."
    echo "Install with: npm install -g wrangler"
    echo ""
fi

# Build the CLI
echo "Building CLI tool..."
cargo build --bin lastfm-cli --release

# Install the CLI
echo "Installing lastfm-cli..."
cargo install --path . --bin lastfm-cli --force

# Create config directory
CONFIG_DIR="$HOME/.config/lastfm-cli"
mkdir -p "$CONFIG_DIR"

# Check if installation was successful
if command -v lastfm-cli &> /dev/null; then
    echo ""
    echo "✅ Installation successful!"
    echo ""
    echo "Quick start:"
    echo "  lastfm-cli --help              # Show help"
    echo "  lastfm-cli artist info \"Queen\" # Get artist info"
    echo "  lastfm-cli interactive         # Start interactive mode"
    echo "  lastfm-cli config list         # View configuration"
    echo ""
    echo "Configuration file location: $CONFIG_DIR/config.toml"
else
    echo "❌ Installation failed. Please check the error messages above."
    exit 1
fi