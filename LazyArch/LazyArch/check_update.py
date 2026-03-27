import subprocess
from InquirerPy import inquirer

REPO_URL = "https://github.com/MarceloAntonio/LazyArch"

def check_update():
    try:
        subprocess.run(
            ["git", "fetch", "origin", "main"],
            check=True,
            capture_output=True
        )
       
        local = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            capture_output=True, text=True, check=True
        ).stdout.strip()

        remote = subprocess.run(
            ["git", "rev-parse", "origin/main"],
            capture_output=True, text=True, check=True
        ).stdout.strip()

        if local == remote:
            print("✓ LazyArch is up to date!")
            return

        print(f"Update available!")
        confirm = inquirer.confirm(
            message="Do you want to update LazyArch?",
            default=True
        ).execute()

        if confirm:
            _do_update()

    except subprocess.CalledProcessError:
        print("Could not check for updates. Are you connected to the internet?")


def _do_update():
    try:
        subprocess.run(["git", "pull", "origin", "main"], check=True)
        print("✓ LazyArch updated successfully! Restart to apply changes.")
    except subprocess.CalledProcessError:
        print("Update failed. Try manually: git pull origin main")