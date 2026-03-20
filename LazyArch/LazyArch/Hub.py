from . import EncerrarPrograma, InstallAUR, InstallLazyVim, ChangeShell, clean, AddUser
from InquirerPy import inquirer


def Hub():
      while(True):
            hubChoice = inquirer.select(
                  message="Select an option:",
                  choices=["Install AUR", "Install & change shell", "Install LazyVim","Create User","Exit"],
            ).execute()
            match hubChoice:
                        case "Install AUR":
                              clean.clean()
                              InstallAUR.InstallAUR()
                        case "Install & change shell":
                              clean.clean()
                              ChangeShell.ChangeShell()
                        case "Install LazyVim":
                              clean.clean()
                              InstallLazyVim.InstallLazyVim()
                        case "Create User":
                              clean.clean()
                              AddUser.AddUser()

                        case "Exit":
                              clean.clean()
                              EncerrarPrograma.EncerrarPrograma("Closing")

                        case _:
                              print("Unknown choice")