# Use an Ubuntu base image.  This is a common and stable base for Rust
FROM ubuntu:22.04

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
    libssl-dev \
    lsb-release \
    software-properties-common \
    gnupg

# Install LLVM 19
RUN curl -L https://apt.llvm.org/llvm.sh | bash -s 19

# Set environment variables for LLVM.  This is crucial for Rust to find it.
ENV LLVM_SYS_19_PREFIX="/usr"
ENV PATH="$PATH:/usr/lib/llvm-19/bin"
ENV LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/usr/lib/llvm-19/lib"

# Install Rust using rustup.  We do this *after* LLVM installation.
RUN curl -sSf https://sh.rustup.rs | bash -s -- -y

# Add Rust binaries to the PATH.  We'll configure it to use LLVM 19.
ENV PATH="/app/.cargo/bin:$PATH"

# Install Rust.
RUN rustup toolchain install
RUN rustup default
RUN cargo install rustfilt

# Configure Rust to use the system-installed LLVM 19.  This is the key step.
ENV RUSTFLAGS="-C link-arg=-fuse-ld=lld -C llvm-config=/usr/bin/llvm-config-19"

# Clean up unnecessary files and directories to reduce image size
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Install custom scripts
COPY cover.sh cover.sh

# Verify Installation (Optional, but highly recommended for debugging)

CMD printf "Hello World!\n" \
    rustc --version && \
    rustc -vV && \
    llvm-config-19 --version && \
    dir /app
