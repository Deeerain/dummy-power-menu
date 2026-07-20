FROM archlinux:base-devel

RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm git pkgconf gtk4 gtk4-layer-shell rustup clang

RUN rustup toolchain install stable && \
    rustup default stable && \
    rustup component add clippy rustfmt

WORKDIR /volume