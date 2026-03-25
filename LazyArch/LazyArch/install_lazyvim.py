import subprocess
from InquirerPy import inquirer

def install_lazyvim():

     confirmation = inquirer.text(message="Are you sure you want to install LazyVim? (Y/N)").execute()
     if confirmation.lower() == "y":
          print("\n\n# Installing dependencies #\n\n")
          subprocess.run("sudo pacman -S --noconfirm --needed nvim git ", shell=True)
          print("\n\n# Cloning repository #\n\n")
          subprocess.run("git clone https://github.com/LazyVim/starter ~/.config/nvim", shell=True)
          print("\n\n# Cleaning cache #\n\n")
          subprocess.run("rm -rf ~/.config/nvim/.git", shell=True)
          print("\n\n# Lazy vim installed successfully #\n\n")

     else:
          print("\nReturning to the menu\n") 
    