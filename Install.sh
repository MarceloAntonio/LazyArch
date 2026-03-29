#!/bin/bash
set -e

REPO_URL="https://github.com/MarceloAntonio/LazyArch.git"
INSTALL_DIR="/tmp/LazyArch"

echo "==> Checking system..."

# Check if already installed
if [ -f /usr/local/bin/LazyArch ]; then
    echo "LazyArch is already installed."
    exit 0
fi

# Check if Arch Linux
if [ ! -f /etc/arch-release ]; then
    echo "LazyArch only supports Arch Linux."
    exit 1
fi

# Check if git exists
if ! command -v git &> /dev/null; then
    echo "Installing git..."
    sudo pacman -S git --noconfirm --needed
fi

echo "==> Cloning LazyArch..."
git clone "$REPO_URL" "$INSTALL_DIR"

cd "$INSTALL_DIR"

script="LazyArch/LazyArch.sh"
dependencies="LazyArch/requirements.txt"

# Validate files
if [ ! -f "$script" ]; then
    echo "Error: File $script not found"
    exit 1
fi

echo "==> Preparing installation..."
chmod +x "$script"

# Check Python
check_python() {
    if ! command -v python &> /dev/null; then
        echo "Installing Python..."
        sudo pacman -S python --noconfirm --needed
    else
        echo "Python is already installed"
    fi
}

# Check pip
check_pip() {
    if ! command -v pip &> /dev/null; then
        echo "Installing pip..."
        sudo pacman -S python-pip --noconfirm --needed
    else
        echo "pip is already installed"
    fi
}

check_python
check_pip

echo "==> Installing dependencies..."

if [ -f /.dockerenv ]; then
    python -m pip install -r "$dependencies" --break-system-packages
else
    python -m pip install -r "$dependencies"
fi

echo "==> Installing LazyArch..."

sudo mkdir -p /usr/local/bin/LazyArch_files
sudo cp -a LazyArch/. /usr/local/bin/LazyArch_files/

sudo ln -sf /usr/local/bin/LazyArch_files/LazyArch.sh /usr/local/bin/LazyArch

echo "==> Cleaning up..."
rm -rf "$INSTALL_DIR"

echo ""
echo "======================================="
echo " LazyArch installed successfully!"
echo "======================================="
echo "Run: LazyArch"
echo ""