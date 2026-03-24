

# LazyArch

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

>  **Note:**
> If you encounter errors while using Docker, uncomment the line `#--break-system-packages` on line 43.

---

## Run LazyArch from anywhere

```bash
LazyArch
```

---

## Running with Docker

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

##  Notes

* Make sure you have Docker and Docker Compose installed if using the container method.


