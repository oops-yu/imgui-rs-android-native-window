#!/bin/bash
xargo ndk -t arm64-v8a -p 24  build --features "gpu-allocator" --release -Zbuild-std=panic_abort,std 