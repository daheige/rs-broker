#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

# 启动二进制，其实就是/app/main
RUST_LOG=info $root_dir/main

# 为了模拟等待操作，这样可以进入容器中执行/app/main实现消息发送
sleep 100000
