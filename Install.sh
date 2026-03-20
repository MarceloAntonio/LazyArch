#!/bin/bash


script="./Hub_Installer_Arch/hub_arch.sh"
dependencies="./Hub_Installer_Arch/requirements.txt"


if [ ! -f "$script" ]; then
    echo "Erro: Arquivo $script não encontrado"
    exit 1
fi


sudo chmod +x "$script"

checar_pip() {
    # Verifica se o executável 'pip' existe no sistema
    if ! command -v pip &> /dev/null; then
        echo "Pip não encontrado. Instalando..."
        sudo pacman -S python-pip --noconfirm --needed
    else
        echo ""
    fi
}


checar_python() {
    if pacman -Qi python &> /dev/null; then
        echo ""
    else
        echo "Python não instalado"
        echo "instalando Python"
        sudo pacman -S python --noconfirm --needed
    fi
}

checar_python
checar_pip

pip install -r "$dependencies" --break-system-packages


sudo mkdir -p /usr/local/bin/hub_arch_files


sudo cp -r "./Hub_Installer_Arch/." /usr/local/bin/hub_arch_files/


sudo ln -sf /usr/local/bin/hub_arch_files/hub_arch.sh /usr/local/bin/hub_arch

echo "Pronto! Agora é só digitar hub_arch em qualquer terminal."