import subprocess
from InquirerPy import inquirer


def _password(name):
        print(f"\nCreate a password for {name}\n")
        subprocess.run(["sudo", "passwd",name])

def add_user():
                
                name = inquirer.text(message="Enter your name").execute()
                
                result = subprocess.run(["sudo","useradd", "-m", "-G", "wheel", "-s", "/bin/bash", name])
                
                match result.returncode:
                        case 0: 
                                _password(name)

                        case 9: 
                                print(f"Error: user {name} already exists")
                        case 3: 
                                print(f"Error: invalid name")

                        case _:
                                print(f"Unknown error!")