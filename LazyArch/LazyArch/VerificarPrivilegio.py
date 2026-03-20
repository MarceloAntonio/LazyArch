import os

#Função que verifica se o usuario está usando privilégio como root ou sudo
def VerificarPrivilegio():
        if os.geteuid() == 0:
            return True
        else:
            return False