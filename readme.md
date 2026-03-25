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

LazyArch is a program that automates boring or time-consuming installations and configurations.  
Instead of searching and copying commands multiple times, LazyArch does everything for you — just select an option and it will install what you need along with its dependencies.

We follow the **KISS (Keep It Simple, Stupid)** philosophy, so everything is designed to be simple and hassle-free.

---


## Features

- Install and automatically configure shells (Fish, Zsh, Bash)
- Install LazyVim
- Install AUR
- Create users

---

If you're interested, follow the steps below to install:

## Running on your machine

### 1. Clone the repository
```bash
git clone https://github.com/MarceloAntonio/LazyArch
````

### 2. Enter the project folder

```bash
cd LazyArch
```

### 3. Make the installer executable

```bash
chmod +x Install.sh
```

### 4. Run the installer

```bash
./Install.sh
```

---

## Run LazyArch from anywhere

```bash
LazyArch
```

---

## Running with Docker

If you don’t want to run it directly on your machine, a Docker environment is available so you can test it safely.

### 1. Enter the project folder

```bash
cd LazyArch
```

### 2. Start the container

```bash
docker compose up -d
```

### 3. Access the container

```bash
docker exec -it LazyArch /bin/bash
```

### 4. Run the installer inside the container

Follow the steps from **"Running on your machine"**.

---

## Uninstalling

### 1. Enter the project folder

```bash
cd LazyArch
```

### 2. Make the uninstaller executable

```bash
chmod +x Uninstall.sh
```

### 3. Run the uninstaller

```bash
./Uninstall.sh
```

---

## Notes

* Make sure you have Docker and Docker Compose installed if you are using the container method.
* If a password is required inside the Docker container, use: **1234**.

