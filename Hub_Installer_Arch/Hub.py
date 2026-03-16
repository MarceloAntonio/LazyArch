from . import EncerrarPrograma, InstallAUR, InstallLazyVim, ChangeShell, clean, AddUser

#Função Hub, função que pede a opção e retorna a função pedida
def Hub():
    while(True):
        hubChoice = int(input("HUB\nEscolha uma das opções a baixo\n[1] - Instalar AUR\n[2] - Instalar & trocar shell (BETA)\n[3] - Instalar LazyVim\n[4] - Criar User\n[5] - Sair\n: "))
        match hubChoice:
             case 1:
                   clean.clean()
                   InstallAUR.InstallAUR()
             case 2:
                   clean.clean()
                   ChangeShell.ChangeShell()
             case 3:
                   clean.clean()
                   InstallLazyVim.InstallLazyVim()
             case 4:
                   clean.clean()
                   AddUser.AddUser()

             case 5:
                   clean.clean()
                   EncerrarPrograma.EncerrarPrograma("Encerrando")

             case _:
                  print("Escolha desconhecida")