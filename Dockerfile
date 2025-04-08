FROM rust:1.86-slim as builder

WORKDIR /app

# 定义构建参数，默认使用 std_io 特性
ARG FEATURES=std_io

# 安装依赖
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# 复制项目文件
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# 构建项目 - 根据构建参数选择特性
RUN cargo build --release --no-default-features --features ${FEATURES}

# 使用较小的基础镜像
FROM debian:bookworm-slim

WORKDIR /app

# 安装运行时依赖
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/weather-rmcp /app/

# 设置环境变量
ENV RUST_LOG=info

# 暴露端口 - 对于 SSE 功能需要
EXPOSE 8080

# 运行应用
CMD ["/app/weather-rmcp"] 