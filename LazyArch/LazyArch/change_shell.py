import os
import subprocess
from InquirerPy import inquirer
from .utils import pacman_install

def _configuring_shell(shell, shell_path):
               
               print(f"Installing {shell}")
               pacman_install(shell)
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
                    _configuring_shell("bash","/bin/bash")                   

               case "Zsh":
                   _configuring_shell("zsh zsh-completions", "/usr/bin/zsh")                    
               
               case "Fish":
                    _configuring_shell("fish","/usr/bin/fish")

               case "Back":
                    return

               case _:
                    print("Unknown option")