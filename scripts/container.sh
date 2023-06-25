#!/bin/bash

source config.sh

STEG_DOCKER_IMAGE="steganography:latest"
STEG_IMAGES="/home/ieg/dev/steganography/resources"

docker run --rm -it \
    --privileged \
    -u $(id -u ${USER}):$(id -g ${USER}) \
    -v $STEG_IMAGES:/mnt/images \
    $STEG_DOCKER_IMAGE
