import subprocess
from InquirerPy import inquirer
from .utils import pacman_install

def install_aur():

     install_dir="/tmp/yay"
     confirmation = inquirer.confirm(
        message="Are you sure you want to install AUR yay?",
        default=True
    ).execute()
     if confirmation:
          
          print("\n\n# Installing dependencies #\n\n")
          pacman_install("base-devel", "git")

          print("\n\n# Cloning repository #\n\n")
          subprocess.run(f"git clone https://aur.archlinux.org/yay.git {install_dir}", shell=True)

          print("\n\n# Entering the folder yay #\n\n")
          subprocess.run(f"", shell=True)

          print("\n\n# Running the initial installation #\n\n")
          subprocess.run(f"cd {install_dir} && makepkg -si", shell=True)

          print("\n\n# Cleaning cache #\n\n")
          subprocess.run(f"sudo rm -R {install_dir}", shell=True)

          print("\n\n# AUR yay installed successfully! #\n\n")

     else:
          print("\nReturning to the menu\n")