#!/bin/bash

ROOT_DIR=${1:-.}

echo "clean  bin/obj in $ROOT_DIR..."

find "$ROOT_DIR" -type d \( -name "bin" -o -name "obj" \) -exec rm -rf {} +

echo "ok!"
