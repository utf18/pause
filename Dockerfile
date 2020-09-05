FROM rust

WORKDIR /usr/src/pause
COPY . .

RUN cargo build --release

FROM debian

COPY --from=0 /usr/src/pause/target/release/pause /usr/local/bin/pause

ENTRYPOINT [ "/usr/local/bin/pause" ]
CMD [""]