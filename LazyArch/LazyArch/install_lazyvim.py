import subprocess
from InquirerPy import inquirer
from .utils import pacman_install

def install_lazyvim():

     confirmation = inquirer.confirm(
        message="Are you sure you want to install LazyVim?",
        default=True
    ).execute()
     if confirmation:
     
          print("\n\n# Installing dependencies #\n\n")
          pacman_install("nvim", "git")

          print("\n\n# Cloning repository #\n\n")
          subprocess.run("git clone https://github.com/LazyVim/starter ~/.config/nvim", shell=True)

          print("\n\n# Cleaning cache #\n\n")
          subprocess.run("rm -rf ~/.config/nvim/.git", shell=True)

          print("\n\n# Lazy vim installed successfully #\n\n")

     else:
          print("\nReturning to the menu\n") 
    