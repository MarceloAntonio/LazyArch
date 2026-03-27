# LazyArch

![License](https://img.shields.io/github/license/MarceloAntonio/LazyArch)
![Last Commit](https://img.shields.io/github/last-commit/MarceloAntonio/LazyArch)
![Repo Size](https://img.shields.io/github/repo-size/MarceloAntonio/LazyArch)
![Docker](https://img.shields.io/badge/docker-supported-blue)
![Arch Linux](https://img.shields.io/badge/arch-linux-blue)

<p align="center">
  <img src="assets/logo.png" alt="LazyArch Logo" width="200"/>
</p>

---

## What is LazyArch?

LazyArch is a tool that automates boring or time-consuming installations and configurations.

Instead of searching for commands and copying them repeatedly, LazyArch does everything for you. Just select an option, and it installs what you need along with all required dependencies.

The project follows the **KISS (Keep It Simple, Stupid)** philosophy, aiming for simplicity and a hassle-free experience.

---

## Features

- Install and automatically configure shells (Fish, Zsh, Bash)
- Install LazyVim
- Install AUR helpers
- Create users
- Git Config

---

## Installation

### Option 1: Quick install (recommended)

```bash
curl -sSL https://raw.githubusercontent.com/MarceloAntonio/LazyArch/refs/heads/main/Install.sh | bash
````

---

### Option 2: Manual installation

#### 1. Clone the repository

```bash
git clone https://github.com/MarceloAntonio/LazyArch
```

#### 2. Enter the project folder

```bash
cd LazyArch
```

#### 3. Make the installer executable

```bash
chmod +x Install.sh
```

#### 4. Run the installer

```bash
./Install.sh
```

---

## Usage

After installation, you can run LazyArch from anywhere:

```bash
LazyArch
```

---

## Running with Docker

If you don’t want to install anything on your system, you can run LazyArch using Docker:

```bash
docker run -it marceloantonio505/lazyarch
```        

Then follow the installation steps inside the container.

---

## Updating

Open LazyArch and navigate to:

More → Check Update

---

## Uninstalling

Open LazyArch and navigate to:

More → Uninstall

---

## Notes

* LazyArch is designed for **Arch Linux-based systems**
* Make sure you have **Docker installed** if using the container method
* Default Docker password (if required): **1234**