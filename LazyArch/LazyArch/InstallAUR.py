import subprocess
from InquirerPy import inquirer

def InstallAUR():
     confirmation = inquirer.text(message="\n\nAre you sure you want to install AUR yay? (Y/N): ").execute()

     if confirmation.lower() == "y":
          
          print("\n\n# Installing dependencies #\n\n")
          subprocess.run("sudo pacman -S --noconfirm --needed base-devel git", shell=True)
          print("\n\n# Cloning repository #\n\n")
          subprocess.run("git clone https://aur.archlinux.org/yay.git", shell=True)
          print("\n\n# Entering the folder yay #\n\n")
          subprocess.run("cd yay", shell=True)
          print("\n\n# Running the initial installation #\n\n")
          subprocess.run("cd yay && makepkg -si", shell=True)
          print("\n\n# Cleaning cache #\n\n")
          subprocess.run("sudo rm -R yay", shell=True)
          print("\n\n# AUR yay installed successfully! #\n\n")

     else:
          print("\nReturning to the Hub\n")