#!/bin/bash
set -e

echo "==> Installing LazyArch..."

INSTALL_DIR="/usr/local/bin"
BINARY_NAME="lazy-arch"
REPO="MarceloAntonio/LazyArch"

# Pega a URL do binário mais recente do GitHub Releases
LATEST_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
  | grep "browser_download_url" \
  | grep "lazy-arch" \
  | cut -d '"' -f 4)

if [ -z "$LATEST_URL" ]; then
  echo "Error: Could not find a release binary. Check your GitHub Releases."
  exit 1
fi

echo "==> Downloading $LATEST_URL..."
curl -L "$LATEST_URL" -o "/tmp/$BINARY_NAME"

echo "==> Installing to $INSTALL_DIR..."
sudo install -m 755 "/tmp/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

rm "/tmp/$BINARY_NAME"

echo ""
echo "======================================="
echo " LazyArch installed successfully!"
echo "======================================="
echo "Run: lazy-arch"
echo ""