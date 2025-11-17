#!/bin/bash
# Script to download Rime resources for LingCode

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RESOURCES_DIR="$PROJECT_ROOT/resources"
TEMP_DIR="$RESOURCES_DIR/temp"

echo "=== LingCode Resources Downloader ==="
echo "This script will download resources from Rime repositories"
echo ""

# Create directories
mkdir -p "$RESOURCES_DIR/schemas"
mkdir -p "$RESOURCES_DIR/dicts"
mkdir -p "$RESOURCES_DIR/opencc"
mkdir -p "$TEMP_DIR"

cd "$TEMP_DIR"

# Download rime-prelude
echo "Downloading rime-prelude..."
if [ ! -d "rime-prelude" ]; then
    git clone --depth 1 https://github.com/rime/rime-prelude.git
fi

# Download rime-luna-pinyin
echo "Downloading rime-luna-pinyin..."
if [ ! -d "rime-luna-pinyin" ]; then
    git clone --depth 1 https://github.com/rime/rime-luna-pinyin.git
fi

# Download rime-double-pinyin
echo "Downloading rime-double-pinyin..."
if [ ! -d "rime-double-pinyin" ]; then
    git clone --depth 1 https://github.com/rime/rime-double-pinyin.git
fi

# Download rime-opencc (if needed)
echo "Downloading rime-opencc..."
if [ ! -d "rime-opencc" ]; then
    git clone --depth 1 https://github.com/rime/rime-opencc.git
fi

# Copy schema files
echo "Copying schema files..."
find rime-prelude -name "*.yaml" -exec cp {} "$RESOURCES_DIR/schemas/" \;
find rime-luna-pinyin -name "*.schema.yaml" -exec cp {} "$RESOURCES_DIR/schemas/" \;
find rime-double-pinyin -name "*.schema.yaml" -exec cp {} "$RESOURCES_DIR/schemas/" \;

# Copy dictionary files
echo "Copying dictionary files..."
find rime-luna-pinyin -name "*.dict.yaml" -exec cp {} "$RESOURCES_DIR/dicts/" \;

# Copy OpenCC files
echo "Copying OpenCC files..."
if [ -d "rime-opencc" ]; then
    cp -r rime-opencc/* "$RESOURCES_DIR/opencc/" 2>/dev/null || true
fi

echo ""
echo "=== Download Complete ==="
echo "Resources have been downloaded to: $RESOURCES_DIR"
echo ""
echo "Directory contents:"
echo "  - schemas: $(find "$RESOURCES_DIR/schemas" -name "*.yaml" | wc -l) files"
echo "  - dicts: $(find "$RESOURCES_DIR/dicts" -name "*.yaml" | wc -l) files"
echo ""
echo "You can now build the project with: cargo build --workspace"

# Optional: Clean up temp directory
read -p "Clean up temporary files? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Cleaning up..."
    rm -rf "$TEMP_DIR"
    echo "Done!"
fi
