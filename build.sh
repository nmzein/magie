#!/bin/bash

if [ "$1" = "prod" ]; then
    eval "
        cd backend && cargo build --release & \
        cd backend/geometry-computer/ && bun install & \
        cd frontend && bun install
        "
elif [ "$1" = "dev" ]; then
    eval "
        cd backend && cargo build & \
        cd backend/geometry-computer/ && bun install & \
        cd frontend && bun install
        "
else
    echo "Usage: ./build.sh [prod|dev]"
fi