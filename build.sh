#!/bin/bash
# rm -rf target

xargo ndk -t arm64-v8a -p 24  build --features "gpu-allocator" --release -Zbuild-std 