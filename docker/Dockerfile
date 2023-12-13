FROM ghcr.io/rust-cross/manylinux2014-cross:aarch64

# 安装交叉编译工具和必要的库
RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu wget && \
    rm -rf /var/lib/apt/lists/*

# 设置交叉编译器环境变量
ENV CC=aarch64-linux-gnu-gcc
ENV CXX=aarch64-linux-gnu-g++

# 下载并编译特定版本的 OpenSSL
WORKDIR /tmp
RUN wget https://www.openssl.org/source/openssl-3.2.0.tar.gz
RUN tar xzf openssl-3.2.0.tar.gz
RUN cd openssl-3.2.0 && ./Configure linux-aarch64 shared --prefix=/usr/local/ssl
RUN cd openssl-3.2.0 && make
RUN cd openssl-3.2.0 && make install

# 设置环境变量
ENV OPENSSL_LIB_DIR=/usr/local/ssl/lib
ENV OPENSSL_INCLUDE_DIR=/usr/local/ssl/include