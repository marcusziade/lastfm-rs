# Favicon Information

This directory contains the favicon files for the lastfm-rs project website.

## Files

- **favicon.svg** - The main favicon in SVG format that combines:
  - Rust's gear/cog symbol in Last.fm's signature red color (#d32f2f)
  - A stylized "R" in the center representing Rust
  - Last.fm's wave pattern in the bottom-right corner
  - Black background for good contrast

- **generate-favicon.sh** - Script to generate favicon.ico from the SVG file

## Design Rationale

The favicon design merges two visual identities:
1. **Rust Language**: The gear/cog shape is a recognizable symbol for Rust
2. **Last.fm**: The red color scheme and wave pattern represent Last.fm's branding

## Generating ICO File

To generate a favicon.ico file from the SVG:

```bash
cd docs
./generate-favicon.sh
```

This requires ImageMagick to be installed:
- Ubuntu/Debian: `sudo apt-get install imagemagick`
- macOS: `brew install imagemagick`
- Fedora: `sudo dnf install ImageMagick`

The script will create a multi-resolution ICO file with 16x16, 32x32, and 48x48 pixel versions.

## Browser Support

The HTML references both SVG and ICO formats:
- Modern browsers will use the SVG version for better scaling
- Older browsers will fall back to the ICO file
- Apple devices will use the SVG as a touch icon
- The theme color is set to match the Last.fm red (#d32f2f)