#!/bin/bash


script="./Hub_Installer_Arch/hub_arch.sh"


if [ ! -f "$script" ]; then
    echo "Erro: Arquivo $script não encontrado"
    exit 1
fi


sudo chmod +x "$script"


sudo mkdir -p /usr/local/bin/hub_arch_files


sudo cp -r "./Hub_Installer_Arch/." /usr/local/bin/hub_arch_files/


sudo ln -sf /usr/local/bin/hub_arch_files/hub_arch.sh /usr/local/bin/hub_arch

echo "Pronto! Agora é só digitar hub_arch em qualquer terminal."