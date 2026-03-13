import subprocess

def InstalarNeoVimPlus():
     
     confirmation = input("Você tem certeza que deseja instalar o Lazy vim? (S/N)")

     if confirmation.lower() == "s":
          
          subprocess.run("sudo pacman -S --needed nvim git ", shell=True)
          subprocess.run("git clone https://github.com/LazyVim/starter ~/.config/nvim", shell=True)
          subprocess.run("rm -rf ~/.config/nvim/.git", shell=True)
          
    