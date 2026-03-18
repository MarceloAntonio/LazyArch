from . import EncerrarPrograma, InstallAUR, InstallLazyVim, ChangeShell, clean, AddUser
from InquirerPy import inquirer


def Hub():
      while(True):
            hubChoice = inquirer.select(
                  message="Selecione uma opção:",
                  choices=["Instalar AUR", "Instalar & trocar shell (BETA)", "Instalar LazyVim","Criar User","Sair"],
            ).execute()
            match hubChoice:
                        case "Instalar AUR":
                              clean.clean()
                              InstallAUR.InstallAUR()
                        case "Instalar & trocar shell (BETA)":
                              clean.clean()
                              ChangeShell.ChangeShell()
                        case "Instalar LazyVim":
                              clean.clean()
                              InstallLazyVim.InstallLazyVim()
                        case "Criar User":
                              clean.clean()
                              AddUser.AddUser()

                        case "Sair":
                              clean.clean()
                              EncerrarPrograma.EncerrarPrograma("Encerrando")

                        case _:
                              print("Escolha desconhecida")