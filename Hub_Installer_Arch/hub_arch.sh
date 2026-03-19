#!/bin/bash

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
    local pacote=$1
    if pacman -Qi "$pacote" &> /dev/null; then
        echo ""
    else
        echo "Python não instalado"
        echo "instalando Python"
        sudo pacman -S python --noconfirm --needed
    fi
}

checar_python
checar_pip

pip install -r requirements.txt --break-system-packages
python main.py


