import os

def check_privileges():
        if os.geteuid() == 0:
            return True
        else:
            return False