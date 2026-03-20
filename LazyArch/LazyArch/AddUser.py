import subprocess
from InquirerPy import inquirer


def senha(name):
        print(f"\nCreate a password for {name}\n")
        subprocess.run(["sudo", "passwd",name])

def AddUser():
                
                name = inquirer.text(message="Enter your name").execute()
                
                resultado = subprocess.run(["sudo","useradd", "-m", "-G", "wheel", "-s", "/bin/bash", name])
                
                match resultado.returncode:
                        case 0: 
                                senha(name)

                        case 9: 
                                print(f"Error: user {name} already exists")
                        case 3: 
                                print(f"Error: invalid name")

                        case _:
                                print(f"Unknown error!")