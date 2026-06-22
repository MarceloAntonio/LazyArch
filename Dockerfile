FROM archlinux:latest

# Atualiza e instala rust + dependências
RUN pacman -Syu --noconfirm && pacman -S --noconfirm \
    sudo \
    curl \
    && pacman -Sc --noconfirm

# Cria usuário dev e define a senha
RUN useradd -m -s /bin/bash dev && \
    echo "dev:dev" | chpasswd && \
    echo "dev ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

WORKDIR /home/dev

USER dev

RUN curl -sSL https://raw.githubusercontent.com/MarceloAntonio/LazyArch/refs/heads/main/Install.sh | bash

CMD ["bash"]