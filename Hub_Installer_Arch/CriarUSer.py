import os
import subprocess
def CriarUser():
                name = input("Coloque seu nome\n:");
                subprocess.run(["doas","useradd", "-m", "-G", "users,wheel", "-s", "/bin/bash", name])

                print(f"\nCriei uma senha para {name}\n")
                subprocess.run(["passwd",name])

                print("usuario criado")

                entrar = input("Deseja entrar no usuario? (S/N)")
                if entrar.lower() == "s":
                    print("Entrando no usuario")
                    print(name)
                    os.execlp("su", "su", "-", name)