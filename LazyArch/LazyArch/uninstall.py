import os
import sys
import subprocess
from InquirerPy import inquirer
from . import cleanup
from . import exit_program



def uninstall():
    lazyarch_dir = "/usr/local/bin/LazyArch_files"
    lazyarch_exec = "/usr/local/bin/LazyArch"

    if not os.path.isdir(lazyarch_dir) and not os.path.exists(lazyarch_exec):
        print("LazyArch is already uninstalled. Nothing to do.")
        sys.exit(0)

    confirmation = inquirer.text(message="Are you sure you want to unistall? (Y/N)").execute()
    if confirmation.lower() == "y":
            print("Uninstalling LazyArch...")

            try:
                subprocess.run(
                    ["sudo", "rm", "-rf", lazyarch_dir, lazyarch_exec],
                    check=True
                )
                cleanup.cleanup()
                exit_program.exit_program("LazyArch has been successfully removed.")

            except subprocess.CalledProcessError as e:
                print("An error occurred during uninstallation.")
                print(f"Error: {e}")
                sys.exit(1)