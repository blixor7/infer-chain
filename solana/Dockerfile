ARG RUST_IMAGE_TAG
ARG SOLANA_CLI

FROM --platform=linux/amd64 kquirapas/solana-builder:v0.3.0

ARG ROOTDIR=.

COPY . .

WORKDIR ${ROOTDIR}

CMD ["cargo", "build-sbf"]
