import os

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