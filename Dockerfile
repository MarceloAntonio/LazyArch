FROM archlinux:latest

# Atualiza e instala rust + dependências
RUN pacman -Syu --noconfirm && pacman -S --noconfirm \
    rust \
    cargo \
    git \
    sudo \
    vim \
    curl \
    base-devel \
    && pacman -Sc --noconfirm

# Cria usuário dev
RUN useradd -m -s /bin/bash dev && \
    echo "dev ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

WORKDIR /app

COPY . .

RUN chown -R dev:dev /app

USER dev

CMD ["bash"]