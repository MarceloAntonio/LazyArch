import getpass
import sys
import os
import subprocess


def EncerrarPrograma(Mensagem):
     print(Mensagem)
     sys.exit()

def InstalarAUR():
     print("Instalar AUR")
     print("Clonando repositório")
     subprocess.run("git clone https://aur.archlinux.org/yay.git", shell=True)
     print("Entrando na pasta yay")
     subprocess.run("cd yay", shell=True)
     print("Rodando o inicializando a instalação")
     subprocess.run("cd yay && makepkg -si", shell=True)
     print("Limpando cache")
     subprocess.run("sudo rm -R yay", shell=True)
     


def VerificarPrivilegio():
        if os.geteuid() == 0:
            return True
        else:
            
            return False
        

def TrocarShell():
     shell = os.environ.get('SHELL')
     print(f"Shell padrão: {shell}") 
     shellAlterar = int(input("Para qual Shell deseja alterar\nBash [1]\nZsh [2]\nFish [3]\nVoltar [4]"))
     match shellAlterar:
          case 1:
               print("Trocar para Bash")
          case 2:
               print("Trocar para Zsh")
               oh_my_zsh = input("Você deseja instalar oh my zsh?")
          case 3:
               print("Trocar para Fish")
          case _:
               print("Opção desconhecida")

def InstalarNeoVimPlus():
     Vim = int(input("Você deseja instalar: \nLazyVim [1]\nLunarVim [2]\nVoltar [3]"))
     match Vim:
          case 1:
               print("Instalar LazyVim")
          case 2:
               print("Instalar LunarVim")
          case _:
               print("Opção desconhecida")




def Hub():
    while(True):
        hub = int(input("Bem vindo ao Hub de programas escolha oq deseja fazer\nInstalar AUR [1]\nTrocar Shell [2]\nInstalar LunarVim ou LazyVim [3]\nSair [4]"))
        match hub:
             case 1:
                   InstalarAUR()
             case 2:
                   TrocarShell()
             case 3:
                   InstalarNeoVimPlus()
             case 4:
                   EncerrarPrograma("Encerrando")
             case _:
                  print("Escolha desconhecida")

     





username = getpass.getuser()

def Setup():

    

        info = input("Você já contém algum usuario? (S/N)")
        if info.lower() == "n":
                name = input("Coloque seu nome\n:");
                subprocess.run(["doas","useradd", "-m", "-G", "users,wheel", "-s", "/bin/bash", name])

                print(f"\nCriei uma senha para {name}\n")
                subprocess.run(["passwd",name])

                print("usuario criado")
                sudo = input("Você deseja que este ususario tenha permissão de usar o doas(Sudo)? (S/N)")
                if sudo.lower() == "s":
                    print("Adicionando usuario Doas(Sudo)")
                    with open(f"/etc/doas.conf", "a", encoding='utf-8') as arquivo:
                        arquivo.write(f"permit {name}\n")


                entrar = input("Deseja entrar no usuario? (S/N)")
                if entrar.lower() == "s":
                    print("Entrando no usuario")
                    print(name)
                    os.execlp("su", "su", "-", name)

                    hub = input("Você deseja ir para o Hub?")
                    if hub.lower() == "s":
                         Hub()
                    else: 
                         EncerrarPrograma("Encerrando")
        else:
                Hub()

Setup()