import subprocess
from InquirerPy import inquirer

def InstallAUR():
     confirmation = inquirer.text(message="\n\n# Você tem certeza que deseja instalar o AUR yay? #\n\n (S/N):").execute()

     if confirmation.lower() == "s":
          print("\n\n# Começando instalação do yay AUR #\n\n")
          print("\n\n# Instalando dependencias #\n\n")
          subprocess.run("sudo pacman -S --noconfirm --needed base-devel git", shell=True)
          print("\n\n# Clonando repositório #\n\n")
          subprocess.run("git clone https://aur.archlinux.org/yay.git", shell=True)
          print("\n\n# Entrando na pasta yay #\n\n")
          subprocess.run("cd yay", shell=True)
          print("\n\n# Rodando o inicializando a instalação #\n\n")
          subprocess.run("cd yay && makepkg -si", shell=True)
          print("\n\n# Limpando cache #\n\n")
          subprocess.run("sudo rm -R yay", shell=True)
          print("\n\n# AUR yay isntalado com sucesso! #\n\n")

     else:
          print("\nRetornando pro Hub\n")