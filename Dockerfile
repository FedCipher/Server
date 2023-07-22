FROM docker.io/library/rust:1.71.0 as BUILDER

WORKDIR /build

COPY . .

RUN cargo build --release

FROM docker.io/library/ubuntu:22.04

ARG USER_ID=1000
ARG GROUP_ID=1000
ARG USERNAME=ciphersafe

ENTRYPOINT [ "/usr/bin/ciphersafe" ]

RUN groupadd --gid $GROUP_ID $USERNAME \
    && \
    useradd --gid $GROUP_ID --uid $USER_ID $USERNAME

COPY --from=BUILDER /build/target/release/server /usr/bin/ciphersafe

USER $USERNAME
