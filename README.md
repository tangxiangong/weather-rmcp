# Weather MCP 服务

这是一个基于 MCP (Model Context Protocol) 的天气服务应用。

## 使用 Docker 构建和运行

### 构建 Docker 镜像

默认使用 `std_io` 特性构建：
```bash
docker build -t weather-mcp-rs .
```

指定特性构建（例如使用 `sse` 特性）：
```bash
docker build --build-arg FEATURES=sse -t weather-mcp-rs .
```

### 运行 Docker 容器

```bash
docker run -p 8080:8080 weather-mcp-rs
```

## 功能特性

- 提供天气预报数据
- 支持标准输入/输出（std_io）和 SSE 传输方式
- 实时获取最新天气数据

## 特性配置

本项目支持两种运行模式：
- **std_io**：默认的应用特性，使用标准输入/输出
- **sse**：使用 Server-Sent Events 进行数据传输

Dockerfile 支持通过构建参数动态选择特性。默认使用 `std_io` 特性，但可以在构建时通过 `--build-arg` 指定不同的特性：

```bash
# 使用默认特性 (std_io) 构建
docker build -t weather-mcp-rs .

# 明确指定 std_io 特性构建
docker build --build-arg FEATURES=std_io -t weather-mcp-rs .

# 使用 sse 特性构建
docker build --build-arg FEATURES=sse -t weather-mcp-rs .
```
