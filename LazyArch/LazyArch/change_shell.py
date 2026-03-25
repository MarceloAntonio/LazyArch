import os
import subprocess
from InquirerPy import inquirer


def configuring_shell(shell, shell_path):
               print(f"Installing {shell}")
               subprocess.run(f"sudo pacman -S --noconfirm --needed {shell}", shell=True)
               print(f"Switching to {shell}")
               subprocess.run(f"chsh -s {shell_path}", shell=True)
               print("Shell changed successfully; restart or log back into your user")


def change_shell():
     shell = os.environ.get('SHELL')
     print(f"Default shell: {shell}") 
 
     shell_options = inquirer.select(
     message="Select an option:",
     choices=["Bash","Zsh", "Fish", "Back"],
     ).execute()
     match shell_options:
               case "Bash":
                    configuring_shell("bash","/bin/bash")
                    

               case "Zsh":
                    print(f"Installing Zsh")
                    subprocess.run("sudo pacman -S --noconfirm --needed zsh zsh-completions", shell=True)
                    print("Setting Zsh as default shell:")
                    subprocess.run("chsh -s /usr/bin/zsh", shell=True)
                    print("Shell changed successfully; restart or log back into your user")
               

               case "Fish":
                    configuring_shell("fish","/usr/bin/fish")


               case "Back":
                    return


               case _:
                    print("Unknown option")