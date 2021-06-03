#!/bin/bash

PROJ_PATH="$(dirname "$(realpath "${BASH_SOURCE[0]}")")/.."
WORK_DIR="$(mktemp -d)"
VERSION="${1:-"dev"}"

cp -f $PROJ_PATH/docker/files/Dockerfile $WORK_DIR
cp -f $PROJ_PATH/docker/files/startup.sh $WORK_DIR
cp -f $PROJ_PATH/docker/files/sources.list $WORK_DIR
cp -f $PROJ_PATH/docker/files/config.properties.template $WORK_DIR
cp -r $PROJ_PATH/target/release $WORK_DIR

sudo docker build $WORK_DIR -t basic-service:$VERSION

rm -rf $WORK_DIR