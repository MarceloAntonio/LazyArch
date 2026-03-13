from . import EncerrarPrograma, InstalarAUR, InstalarNeoVimPlus, TrocarShell

#Função Hub, função que pede a opção e retorna a função pedida
def Hub():
    while(True):
        hubChoice = int(input("HUB\nEscolha uma das opções a baixo\n[1] - Instalar AUR\n[2] - Instalar & trocar shell\nInstalar nvim personalizado\n\n: "))
        match hubChoice:
             case 1:
                   InstalarAUR.InstalarAUR()
             case 2:
                   TrocarShell.TrocarShell()
             case 3:
                   InstalarNeoVimPlus.InstalarNeoVimPlus()
             case 4:
                   EncerrarPrograma.EncerrarPrograma("Encerrando")
             case _:
                  print("Escolha desconhecida")