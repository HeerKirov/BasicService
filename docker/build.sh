#!/bin/bash

PROJ_PATH="$(dirname "$(realpath "${BASH_SOURCE[0]}")")/.."
REGISTRY_PATH=$HOME/.cargo/registry
RUST_IMAGE=rust:1.51

sudo docker run -it --rm \
    --user "$(id -u)":"$(id -g)" \
    -v "$PROJ_PATH":/usr/src/myrust \
    -v "$REGISTRY_PATH":/usr/local/cargo/registry \
    -v "$PROJ_PATH/docker/files/cargo-config":/usr/local/cargo/config \
    -w /usr/src/myrust \
    $RUST_IMAGE \
    cargo build --release