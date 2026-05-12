#!/bin/bash
set -e

BINARY_NAME="lazy-arch"
INSTALL_DIR="/usr/local/bin"

echo "==> Uninstalling LazyArch..."

if [ ! -f "$INSTALL_DIR/$BINARY_NAME" ]; then
  echo "LazyArch is not installed at $INSTALL_DIR/$BINARY_NAME"
  exit 1
fi

sudo rm "$INSTALL_DIR/$BINARY_NAME"

echo ""
echo "======================================="
echo " LazyArch uninstalled successfully!"
echo "======================================="
echo ""