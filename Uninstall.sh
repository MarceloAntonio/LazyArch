if [ ! -d "/usr/local/bin/LazyArch_files" ] && [ ! -d "/usr/local/bin/LazyArch" ]; then
    echo "LazyArch is already uninstalled. Nothing to do."
    exit 0
fi

echo "Uninstalling LazyArch..."

if sudo rm -rf /usr/local/bin/LazyArch_files /usr/local/bin/LazyArch; then
    echo "LazyArch has been successfully removed."
else
    echo "An error occurred during uninstallation."
    exit 1
fi