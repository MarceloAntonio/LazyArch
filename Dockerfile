from archlinux:latest 

RUN pacman-key --init && \
    pacman -Syu --noconfirm --needed \
    base-devel \
    vim \
    git \
    shadow

RUN useradd -m -G wheel -s /bin/bash LazyArch
RUN echo "LazyArch:1234" | chpasswd
RUN echo "%wheel ALL=(ALL:ALL) NOPASSWD: ALL" >> /etc/sudoers

USER LazyArch
WORKDIR /home/LazyArch
RUN git clone https://github.com/MarceloAntonio/LazyArch

