# Create a steganography image. This Dockerfile will build an Alpine Linux
# image which includes a build of the steganography binary.

# This Dockerfile uses a multi-stage build. The first stage, builder, builds
# the steganography binary. The second stage copies the steganography binary
# from builder to the final image.

FROM alpine:latest AS builder

# Install all the packages needed to build steganography.
RUN apk add --no-cache \
        build-base \
        boost-dev \
        libjpeg-turbo-dev \
        libpng-dev \
        cmake \
        bash

# Copy the steganography source tree.
COPY ./ /steganography/

# Build steganography using the build script included in the source tree.
WORKDIR /steganography/scripts
RUN chmod +x clean.sh && \
    ./clean.sh && \
    chmod +x build.sh && \
    ./build.sh

FROM alpine:latest

RUN apk add --no-cache \
        bash \
        libjpeg-turbo \
        libpng \
        libgcc \
        libstdc++

# Copy the steganography binary from the builder image to this final image.
COPY --from=builder /steganography/bin/steganography /usr/local/bin
