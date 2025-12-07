FROM rs-dev:v1.0 AS builder

LABEL authors="daheige"

# 设置环境变量LANG
ENV LANG=C.UTF-8

ENV PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
ENV PKG_CONFIG_ALLOW_SYSTEM_LIBS=1
ENV PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1

WORKDIR /app

COPY . .

# 编译构建rust应用程序
RUN cd /app && cargo build --release

# 将上面构建好的二进制文件复制到容器中运行
FROM debian:bullseye-slim

WORKDIR /app

# 设置gRPC微服务和metrics服务运行端口
EXPOSE 50051
EXPOSE 8090

# 设置deb镜像源，这里我使用aliyun的镜像
RUN echo "deb http://mirrors.aliyun.com/debian bullseye main" > /etc/apt/sources.list &&  \
    echo "deb http://mirrors.aliyun.com/debian-security bullseye-security main" >> /etc/apt/sources.list &&  \
    echo "deb http://mirrors.aliyun.com/debian bullseye-updates main" >> /etc/apt/sources.list &&  \
    apt-get update && apt-get install -y gcc cmake make libtool wget ca-certificates  \
    vim bash curl net-tools iputils-ping apt-transport-https  \
    build-essential libcurl4-openssl-dev libssl-dev libsasl2-dev libzstd-dev zlib1g-dev pkg-config &&  \
    update-ca-certificates && apt-get clean &&  \
    rm -rf /var/lib/apt/lists/* && mkdir -p /app/bin

## 安装rdkafka
#RUN cd /opt && wget https://github.com/confluentinc/librdkafka/archive/refs/tags/v2.12.1.tar.gz && \
#    tar -zxf v2.12.1.tar.gz && cd /opt/librdkafka-2.12.1 && mkdir build && cd build && cmake .. && \
#    make && make install

COPY --from=builder /opt/v2.12.1.tar.gz /opt/v2.12.1.tar.gz
RUN cd /opt && tar -zxf v2.12.1.tar.gz && cd /opt/librdkafka-2.12.1 && mkdir build && cd build && cmake .. && \
    make && make install

# 设置环境变量
ENV PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
ENV PKG_CONFIG_ALLOW_SYSTEM_LIBS=1
ENV PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1

# 验证rdkafka是否安装
#RUN pkg-config --modversion rdkafka

# 将构建阶段的二进制文件复制到工作目录中
COPY --from=builder /app/target/release/rs-broker /app/main
COPY ./bin/entrypoint.sh /app/bin/entrypoint.sh
# 添加执行权限
RUN chmod +x /app/bin/entrypoint.sh

ENTRYPOINT ["/app/bin/entrypoint.sh"]