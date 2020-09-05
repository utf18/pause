FROM rust
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new pause
WORKDIR /usr/src/pause
COPY Cargo.toml ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=0 /usr/local/cargo/bin/pause /usr/local/bin/pause
USER 1000
ENTRYPOINT ["/usr/local/bin/pause"]
CMD [ "" ]