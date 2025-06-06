FROM rust:1.75.0

ENV HOME="/nonroot"
ENV PATH="${HOME}/.local/share/solana/install/active_release/bin:${PATH}"
ARG HOST_UID
ARG HOST_GID

RUN mkdir nonroot
RUN mkdir workdir
RUN chown $HOST_UID:$HOST_GID ./nonroot
RUN chown $HOST_UID:$HOST_GID ./workdir

USER $HOST_UID:$HOST_GID


# Install Solana tools.
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.18.11/install)"

WORKDIR /nonroot

RUN cargo new temp --lib
WORKDIR /nonroot/temp
RUN printf "\n[lib]\ncrate-type=[\"cdylib\"]\n" >> Cargo.toml
RUN cargo build-sbf

WORKDIR /workdir