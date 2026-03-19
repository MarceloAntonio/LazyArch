import subprocess
from InquirerPy import inquirer


def senha(name):
        print(f"\nCriei uma senha para {name}\n")
        subprocess.run(["sudo", "passwd",name])

def AddUser():
                
                name = inquirer.text(message="Coloque seu nome").execute()
                
                resultado = subprocess.run(["sudo","useradd", "-m", "-G", "wheel", "-s", "/bin/bash", name])
                
                match resultado.returncode:
                        case 0: 
                                senha(name)

                        case 9: 
                                print(f"Erro: o usuario {name} já existe")
                        case 3: 
                                print(f"Erro: nome invalido")

                        case _:
                                print(f"Erro desconhecido!")


               

                    
              