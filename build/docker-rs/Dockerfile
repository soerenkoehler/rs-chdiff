# Use an Ubuntu base image.  This is a common and stable base for Rust
FROM ubuntu:24.04

# Prevent interactive prompts during package installation
ARG DEBIAN_FRONTEND=noninteractive

# Set the working directory
ENV HOME="/app"
WORKDIR $HOME

# Install essential build tools, including curl, which is often needed for Rust installation
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        curl \
        ca-certificates \
        jq \
        less \
        libssl-dev \
        lsb-release \
        software-properties-common \
        gnupg

# Install LLVM 19
RUN curl -L https://apt.llvm.org/llvm.sh | bash -s 19
ENV PATH="$PATH:/usr/lib/llvm-19/bin"

# Install Rust using rustup.  We do this *after* LLVM installation.
RUN curl -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/app/.cargo/bin:$PATH"

# Install Rust.
RUN rustup toolchain install
RUN rustup default
RUN cargo install rustfilt

# Clean up unnecessary files and directories to reduce image size
RUN apt-get clean && \
    rm -rf \
        /var/lib/apt/lists/* \
        /var/tmp/* \
        /tmp/*

# Install custom scripts
COPY cover.sh cover.sh
