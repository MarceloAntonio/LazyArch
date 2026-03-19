import os
import subprocess
from InquirerPy import inquirer


def ChangeShell():
     shell = os.environ.get('SHELL')
     print(f"Shell padrão: {shell}") 
 
     shellAlterar = inquirer.select(
     message="Selecione uma opção:",
     choices=["Bash","Zsh", "Fish", "Voltar"],
     ).execute()
     match shellAlterar:
               case "Bash":
                    print("Instalando bash")
                    subprocess.run("sudo pacman -S --noconfirm --needed bash", shell=True)
                    print("Trocando para o bash")
                    subprocess.run("chsh -s /bin/bash", shell=True)


               case "Zsh":
                    subprocess.run("sudo pacman -S --noconfirm --needed zsh zsh-completions", shell=True)
                    print("Definindo Zsh como shell padrão:")
                    subprocess.run("chsh -s /usr/bin/zsh", shell=True)
               
                    
               case "Fish":
                    subprocess.run("sudo pacman -S --noconfirm --needed fish", shell=True)
                    print("Trocando para Fish")
                    subprocess.run("chsh -s /usr/bin/fish", shell=True)
                    print("Shell trocado com sucesso reiniciei ou relogue no usuário")
               case _:
                    print("Opção desconhecida")