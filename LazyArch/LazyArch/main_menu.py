from . import add_user, change_shell, cleanup, exit_program, install_aur, install_lazyvim, uninstall, git_config
from InquirerPy import inquirer

def _more_options():
       more_options_choice = inquirer.select(
                  message="Select an option:",
                  choices=["Uninstall","Back"],
            ).execute()
       match more_options_choice:
              case "Uninstall":
                     cleanup.cleanup()
                     uninstall.uninstall()
              case "Back":
                     cleanup.cleanup()
                     return
              case _:
                     print("Unknown choice")
              
def main_menu():
      while(True):
            menu_choice = inquirer.select(
                  message="Select an option:",
                  choices=["Install AUR", "Change shell", "Install LazyVim","Add user","Git Config","More","Exit"],
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
                              add_user.add_user()
                        case "Git Config":
                              cleanup.cleanup()
                              git_config.git_config()
                        case "More":
                              cleanup.cleanup()
                              _more_options()
                        case "Exit":
                              cleanup.cleanup()
                              exit_program.exit_program("Closing")

                        case _:
                              print("Unknown choice")