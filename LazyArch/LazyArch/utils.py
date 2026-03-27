import subprocess
import shutil

def pacman_install(*packages):
    missing = [p for p in packages if not shutil.which(p)]
    if not missing:
        print(f"{', '.join(packages)} already installed, skipping...")
        return
    subprocess.run(
        ["sudo", "pacman", "-S", "--noconfirm", "--needed", *missing],
        check=True
    )
