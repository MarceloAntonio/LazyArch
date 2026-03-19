from Hub_Installer_Arch import Hub, VerificarPrivilegio, EncerrarPrograma, clean






#Função central que inicia o sistema
def Main():
     if VerificarPrivilegio.VerificarPrivilegio() == True:
          EncerrarPrograma.EncerrarPrograma("Rode o script sem sudo ou root")
     else:
          Hub.Hub()

if __name__ == "__main__":
    clean.clean()
    Main()

