#!/bin/bash


script="./LazyArch/LazyArch.sh"
dependencies="./LazyArch/requirements.txt"


if [ ! -f "$script" ]; then
    echo "Error: File $script not found"
    exit 1
fi

echo "Installing dependencies"
sudo chmod +x "$script"

checar_pip() {
    if ! command -v pip &> /dev/null; then
        echo "Pip not found. Installing..."
        sudo pacman -S python-pip --noconfirm --needed
    else
        echo "Python is already installed"
    fi
}


checar_python() {
    if pacman -Qi python &> /dev/null; then
        echo "Python is already installed"
    else
        echo "Python not found. Installing..."
        
        sudo pacman -S python --noconfirm --needed
    fi
}

checar_python

checar_pip


echo "Installing dependencies"
#Uncomment (#--break-system-packages) in case of error using Docker.
pip install -r "$dependencies" #--break-system-packages


sudo mkdir -p /usr/local/bin/LazyArch_files

sudo cp -r "./LazyArch/." /usr/local/bin/LazyArch_files/

sudo ln -sf /usr/local/bin/LazyArch_files/LazyArch.sh /usr/local/bin/LazyArch

echo "Done! Now just type LazyArch in any terminal."