import subprocess
from InquirerPy import inquirer

def InstallLazyVim():

     confirmation = inquirer.text(message="Você tem certeza que deseja instalar o Lazy vim? (S/N)").execute()
     if confirmation.lower() == "s":
          print("\n\n# Instalando dependencias #\n\n")
          subprocess.run("sudo pacman -S --noconfirm --needed nvim git ", shell=True)
          print("\n\n# Clonando repositório #\n\n")
          subprocess.run("git clone https://github.com/LazyVim/starter ~/.config/nvim", shell=True)
          print("\n\n# Limpando cache #\n\n")
          subprocess.run("rm -rf ~/.config/nvim/.git", shell=True)
          print("\n\n# Lazy vim instalado com sucesso #\n\n")

     else:
          print("\nRetornando pro Hub\n") 
    