from LazyArch import main_menu,cleanup,check_privileges,exit_program

def main():
     if check_privileges.check_privileges() == True:
          exit_program.exit_program("Run the script without sudo or root")
     else:
          main_menu.main_menu()

if __name__ == "__main__":
    cleanup.cleanup()
    main()

