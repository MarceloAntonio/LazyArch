import os
import subprocess




def senha(name):
        print(f"\nCriei uma senha para {name}\n")
        subprocess.run(["sudo", "passwd",name])

def AddUser():
                
                name = input("Coloque seu nome\n:");
                resultado = subprocess.run(["sudo","useradd", "-m", "-G", "wheel", "-s", "/bin/bash", name])
                
                match resultado.returncode:
                        case 0: 
                                senha(name)
                                entrar = input("Deseja entrar no usuario? (S/N)")
                                if entrar.lower() == "s":
                                    print("Entrando no usuario")
                                    print(name)
                                    os.execlp("su", "su", "-", name)

                        case 9: 
                                print(f"Erro: o usuario {name} já existe")
                        case 3: 
                                print(f"Erro: nome invalido")

                        case _:
                                print(f"Erro desconhecido!")


               

                    
              