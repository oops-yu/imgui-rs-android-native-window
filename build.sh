#!/bin/bash
rm -rf target

cargo ndk -t arm64-v8a   build 