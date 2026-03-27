import subprocess
import shutil
import os
from InquirerPy import inquirer
from .utils import pacman_install

def git_config():
    if not shutil.which("git"):
        print("Git not found. Installing...")
        pacman_install("git")

    possible_editors = ["vim", "nvim", "nano"]
    installed_editors = [editor for editor in possible_editors if shutil.which(editor)]

    name = inquirer.text(message="Enter your name:").execute()
    email = inquirer.text(message="Enter your email:").execute()
    core_editor = inquirer.select(
        message="Select a core editor:",
        choices=installed_editors,
    ).execute()

    subprocess.run(["git", "config", "--global", "user.name", name], check=True)
    subprocess.run(["git", "config", "--global", "user.email", email], check=True)
    subprocess.run(["git", "config", "--global", "color.ui", "auto"], check=True)
    subprocess.run(["git", "config", "--global", "init.defaultBranch", "main"], check=True)
    subprocess.run(["git", "config", "--global", "core.editor", core_editor], check=True)

    print("✓ Git configured successfully!")


    generate_ssh = inquirer.confirm(
        message="Do you want to generate an SSH key?",
        default=True
    ).execute()

    if generate_ssh:
        _setup_ssh(email)


def _setup_ssh(email):
    ssh_dir = os.path.expanduser("~/.ssh")
    key_path = os.path.join(ssh_dir, "id_ed25519")

    os.makedirs(ssh_dir, exist_ok=True)

    if os.path.exists(key_path):
        overwrite = inquirer.confirm(
            message="SSH key already exists. Overwrite?",
            default=False
        ).execute()
        if not overwrite:
            _show_public_key(key_path)
            return

    subprocess.run([
        "ssh-keygen", "-t", "ed25519",
        "-C", email,
        "-f", key_path,
        "-N", ""  # sem passphrase
    ], check=True)

    # Inicia o ssh-agent e adiciona a chave
    subprocess.run(["ssh-add", key_path])

    print("\n✓ SSH key generated!")
    _show_public_key(key_path)


def _show_public_key(key_path):
    pub_key_path = key_path + ".pub"
    if os.path.exists(pub_key_path):
        with open(pub_key_path, "r") as f:
            pub_key = f.read().strip()
        print("\n📋 Your public SSH key (add this to GitHub/GitLab):\n")
        print(pub_key)
        print()