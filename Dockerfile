FROM ubuntu:xenial

RUN apt-get update; \
	apt-get install -y wget git gcc

# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    url="https://sh.rustup.rs"; \
    wget -O rustup-init "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN rustup install nightly-2022-08-01
RUN rustup install nightly-2022-08-02

WORKDIR /app
COPY files /app/

RUN chmod +x test.sh

ENTRYPOINT ["bash"]
