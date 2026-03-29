#!/bin/bash
set -e

echo "==> Preparing LazyArch installation..."

# 1. Garante que o usuário tem as ferramentas nativas de compilação do Arch
sudo pacman -S --needed --noconfirm git base-devel

# 2. Cria uma pasta temporária limpa
BUILD_DIR="/tmp/lazyarch_build"
rm -rf "$BUILD_DIR"

# 3. Baixa o seu repositório (que agora tem o PKGBUILD dentro)
git clone https://github.com/MarceloAntonio/LazyArch.git "$BUILD_DIR"

# 4. Entra na pasta e manda o Arch compilar e instalar o pacote silenciosamente
cd "$BUILD_DIR"
echo "==> Building and installing package..."
makepkg -si --noconfirm

# 5. Limpa a sujeira
cd ~
rm -rf "$BUILD_DIR"

echo ""
echo "======================================="
echo " LazyArch installed successfully!"
echo "======================================="
echo "Run: LazyArch"
echo ""