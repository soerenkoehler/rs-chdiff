# use an Ubuntu base image
FROM ubuntu:24.04

ARG USER_ID
ARG GROUP_ID

# prevent interactive prompts during package installation
ARG DEBIAN_FRONTEND=noninteractive

# set up environment and working directories
ENV USER_NAME=docker-rs
ENV USER_HOME="/home/$USER_NAME"

ENV APP_HOME="/app"
ENV WORK_DIR="$APP_HOME/work"
ENV CARGO_HOME="$APP_HOME/.cargo"

ENV LLVM_VERSION=20
ENV LD_LIBRARY_PATH="/usr/lib/llvm-$LLVM_VERSION/lib"
ENV PATHEXT="$CARGO_HOME/bin:/usr/lib/llvm-$LLVM_VERSION/bin"
ENV PATH="$PATHEXT:$PATH"

# set PATH for interactive shell
RUN printf "export PATH=%s:$PATH" "$PATHEXT" >> /etc/bash.bashrc

# set up working directories
WORKDIR $WORK_DIR
WORKDIR $APP_HOME

# install essential tools
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        curl \
        ca-certificates \
        gcc-arm-linux-gnueabi \
        gcc-aarch64-linux-gnu \
        gnupg \
        jq \
        less \
        libc6-dev-armel-cross \
        libc6-dev-arm64-cross \
        libssl-dev \
        lsb-release \
        mingw-w64 \
        nginx \
        software-properties-common \
        vim

# install LLVM
RUN curl -L https://apt.llvm.org/llvm.sh | bash -s $LLVM_VERSION

# clean up unnecessary files and directories to reduce image size
RUN apt-get clean && \
    rm -rf \
        /var/lib/apt/lists/* \
        /var/tmp/* \
        /tmp/*

# set up non-root user
RUN groupadd --gid $GROUP_ID \
             --non-unique \
             $USER_NAME && \
    useradd --shell /bin/false \
            --uid $USER_ID \
            --gid $GROUP_ID \
            --non-unique \
            --create-home \
            --home-dir $USER_HOME \
            $USER_NAME

# prepare nginx to run non-root
RUN touch /var/run/nginx.pid && \
    chown -R $USER_NAME:$USER_NAME \
          $APP_HOME \
          $USER_HOME \
          /var/www \
          /var/lib/nginx \
          /var/log/nginx \
          /var/run/nginx.pid
ENV CONF=/etc/nginx/nginx.conf
RUN mv $CONF $CONF.old && \
    sed -e 's/^user\s/# user /g' $CONF.old >$CONF

# switch to non-root user
USER $USER_NAME

# install Rust using rustup
RUN curl -sSf https://sh.rustup.rs | bash -s -- -y

# Setup Rust
COPY rust-toolchain.toml rust-toolchain.toml
RUN rustup toolchain install
RUN rustup default
RUN cargo install rustfilt

# install custom scripts
COPY *.sh .
