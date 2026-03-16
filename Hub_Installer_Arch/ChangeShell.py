import os
import subprocess

def installStarship():
     print("Instalando starship")
     subprocess.run("pacman -S --noconfirm --needed starship", shell=True)
     print("Ativando starship")
     with open('~/.config/fish/config.fish', 'a', encoding='utf-8') as arquivo:
          arquivo.write("\nstarship init fish | source")

     
     

def installOh_my_zsh():
     
     print("Instalando oh_my_zsh")
     subprocess.run('sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"', shell=True)
     print("Instalando tema Powerlevel10k")
     subprocess.run('git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k', shell=True)
     subprocess.run('sed -i "s/ZSH_THEME=\".*\"/ZSH_THEME=\"powerlevel10k\/powerlevel10k\"/g" ~/.zshrc', shell=True)
     print("Baixando plugins essenciais")
     subprocess.run('git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting', shell=True)





def ChangeShell():
     shell = os.environ.get('SHELL')
     print(f"Shell padrão: {shell}") 
     shellAlterar = int(input("Para qual Shell deseja alterar\nBash [1]\nZsh [2]\nFish [3]\nVoltar [4]"))
     match shellAlterar:
          case 1:
               print("Instalando bash")
               subprocess.run("sudo pacman -S --noconfirm --needed bash", shell=True)
               print("Trocando para o bash")
               subprocess.run("schsh -s /bin/bash", shell=True)
               os.execlp("bash")



          case 2:
               subprocess.run("sudo pacman -S --noconfirm --needed zsh zsh-completions", shell=True)
               confirmation = input("Instalar Oh_My_Zsh? (S/N)\n: ")
               if confirmation.lower() == "s":
                    installOh_my_zsh()
               print("Definindo Zsh como shell padrão:")
               subprocess.run("chsh -s /usr/bin/zsh", shell=True)
               print("entrando no zsh")
               os.execlp("zsh")

               
          case 3:
               subprocess.run("sudo pacman -S --noconfirm --needed fish", shell=True)
               confirmation = input("Instalar Starship? (S/N)\n: ")
               if confirmation.lower() == "s":
                    installStarship()
               
               print("Trocando para Fish")
               subprocess.run("chsh -s /usr/bin/fish", shell=True)
               print("Entrando shell Fish")
          case _:
               print("Opção desconhecida")