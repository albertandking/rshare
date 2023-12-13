# 为了 openssl 编译新镜像
FROM ghcr.io/rust-cross/manylinux2014-cross:aarch64

# 安装交叉编译工具和必要的库
RUN apt-get update && \
    apt-get install -y openssl libssl-dev

# 测试此处是否需要在优化，可以省略？
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/
ENV OPENSSL_INCLUDE_DIR=/usr/include/