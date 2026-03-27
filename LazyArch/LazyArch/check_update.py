import os
import subprocess
from InquirerPy import inquirer

REPO_DIR = os.path.dirname(os.path.abspath(__file__))  # pasta do próprio script

def check_update():
    try:
        subprocess.run(
            ["git", "fetch", "origin", "main"],
            check=True,
            capture_output=True,
            cwd=REPO_DIR  # força rodar na pasta certa
        )

        local = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            capture_output=True, text=True, check=True,
            cwd=REPO_DIR
        ).stdout.strip()

        remote = subprocess.run(
            ["git", "rev-parse", "origin/main"],
            capture_output=True, text=True, check=True,
            cwd=REPO_DIR
        ).stdout.strip()

        if local == remote:
            print("✓ LazyArch is up to date!")
            return

        print("Update available!")
        confirm = inquirer.confirm(
            message="Do you want to update LazyArch?",
            default=True
        ).execute()

        if confirm:
            _do_update()

    except subprocess.CalledProcessError as e:
        print(f"Could not check for updates: {e}")  # agora mostra o erro real


def _do_update():
    try:
        subprocess.run(["git", "pull", "origin", "main"], check=True, cwd=REPO_DIR)
        print("✓ LazyArch updated successfully! Restart to apply changes.")
    except subprocess.CalledProcessError:
        print("Update failed. Try manually: git pull origin main")