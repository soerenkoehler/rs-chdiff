# use an Ubuntu base image
FROM ubuntu:24.04

ARG USER_ID
ARG GROUP_ID

# prevent interactive prompts during package installation
ARG DEBIAN_FRONTEND=noninteractive

# set up the working directory
ENV APP_HOME="/app"
WORKDIR $APP_HOME

# install essential tools
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        curl \
        ca-certificates \
        gnupg \
        jq \
        less \
        libssl-dev \
        lsb-release \
        nginx \
        software-properties-common \
        vim

# install LLVM 20
RUN curl -L https://apt.llvm.org/llvm.sh | bash -s 20
ENV PATH="$PATH:/usr/lib/llvm-20/bin"
ENV LD_LIBRARY_PATH="/usr/lib/llvm-20/lib"

# clean up unnecessary files and directories to reduce image size
RUN apt-get clean && \
    rm -rf \
        /var/lib/apt/lists/* \
        /var/tmp/* \
        /tmp/*

# set up non-root user
ENV USER_NAME=docker-rs
ENV USER_HOME="/home/$USER_NAME"
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
ENV CARGO_HOME="$APP_HOME/.cargo"
RUN curl -sSf https://sh.rustup.rs | bash -s -- -y >/app/rust.log
ENV PATH="$CARGO_HOME/bin:$PATH"

# Setup Rust
COPY rust-toolchain.toml rust-toolchain.toml
RUN rustup toolchain install
RUN rustup default
RUN cargo install rustfilt

# install custom scripts
COPY *.sh .
