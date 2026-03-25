from . import add_user, change_shell, cleanup, exit_program, install_aur, install_lazyvim, uninstall
from InquirerPy import inquirer

def more_options():
       more_options_choice = inquirer.select(
                  message="Select an option:",
                  choices=["Uninstall","Back"],
            ).execute()
       match more_options_choice:
              case "Uninstall":
                     uninstall.uninstall()
              
def main_menu():
      while(True):
            menu_choice = inquirer.select(
                  message="Select an option:",
                  choices=["Install AUR", "Change shell", "Install LazyVim","Add user","More","Exit"],
            ).execute()
            match menu_choice:
                        case "Install AUR":
                              cleanup.cleanup()
                              install_aur.install_aur()
                        case "Change shell":
                              cleanup.cleanup()
                              change_shell.change_shell()
                        case "Install LazyVim":
                              cleanup.cleanup()
                              install_lazyvim.install_lazyvim()
                        case "Add user":
                              cleanup.cleanup()
                              add_user.AddUser()
                        case "More":
                              more_options()
                        case "Exit":
                              cleanup.cleanup()
                              exit_program.exit_program("Closing")

                        case _:
                              print("Unknown choice")