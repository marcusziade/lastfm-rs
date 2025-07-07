#!/bin/bash

# Script to generate favicon.ico from favicon.svg
# Requires ImageMagick (convert command)

# Check if ImageMagick is installed
if ! command -v convert &> /dev/null; then
    echo "ImageMagick is required but not installed."
    echo "Install it with: sudo apt-get install imagemagick (Ubuntu/Debian)"
    echo "or: brew install imagemagick (macOS)"
    exit 1
fi

# Generate multiple sizes for the ICO file
echo "Generating favicon.ico from favicon.svg..."

# Create temporary PNG files at different sizes
convert -background none favicon.svg -resize 16x16 favicon-16.png
convert -background none favicon.svg -resize 32x32 favicon-32.png
convert -background none favicon.svg -resize 48x48 favicon-48.png

# Combine into ICO file
convert favicon-16.png favicon-32.png favicon-48.png favicon.ico

# Clean up temporary files
rm favicon-16.png favicon-32.png favicon-48.png

echo "favicon.ico generated successfully!"