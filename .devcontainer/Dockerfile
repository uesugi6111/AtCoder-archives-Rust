FROM uesugi6111/rust-dev-env:latest

RUN cargo install cargo-compete cargo-member
RUN rustup toolchain install 1.42.0-x86_64-unknown-linux-gnu \
    &&rustup component add rls rust-src rustfmt clippy --toolchain 1.42.0-x86_64-unknown-linux-gnu
