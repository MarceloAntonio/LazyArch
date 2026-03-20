import os
import subprocess
from InquirerPy import inquirer


def ChangeShell():
     shell = os.environ.get('SHELL')
     print(f"Default shell: {shell}") 
 
     shellAlterar = inquirer.select(
     message="Select an option:",
     choices=["Bash","Zsh", "Fish", "Back"],
     ).execute()
     match shellAlterar:
               case "Bash":
                    print("Installing bash")
                    subprocess.run("sudo pacman -S --noconfirm --needed bash", shell=True)
                    print("Switching to bash")
                    subprocess.run("chsh -s /bin/bash", shell=True)


               case "Zsh":
                    subprocess.run("sudo pacman -S --noconfirm --needed zsh zsh-completions", shell=True)
                    print("Setting Zsh as default shell:")
                    subprocess.run("chsh -s /usr/bin/zsh", shell=True)
               
                    
               case "Fish":
                    subprocess.run("sudo pacman -S --noconfirm --needed fish", shell=True)
                    print("Switching to Fish")
                    subprocess.run("chsh -s /usr/bin/fish", shell=True)
                    print("Shell changed successfully; restart or log back into your user")
               case "Back":
                    return
               case _:
                    print("Unknown option")