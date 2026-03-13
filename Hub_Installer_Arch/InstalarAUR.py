import subprocess

def InstalarAUR():
     print("Começando instalação do yay AUR")
     print("Instalando dependencias")
     subprocess.run("sudo pacman -S --needed base-devel git", shell=True)
     print("Clonando repositório\n")
     subprocess.run("git clone https://aur.archlinux.org/yay.git", shell=True)
     print("Entrando na pasta yay\n")
     subprocess.run("cd yay", shell=True)
     print("Rodando o inicializando a instalação\n")
     subprocess.run("cd yay && makepkg -si", shell=True)
     print("Limpando cache\n")
     subprocess.run("sudo rm -R yay", shell=True)
     print("AUR yay isntalado com sucesso!\n")