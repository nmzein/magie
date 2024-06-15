#!/bin/bash

if [ "$1" = "prod" ]; then
    eval "
        cd backend && cargo build --release & \
        cd backend/geometry-computer/ && npm install & \
        cd frontend && npm install --legacy-peer-deps
        "
elif [ "$1" = "dev" ]; then
    eval "
        cd backend && cargo build & \
        cd backend/geometry-computer/ && npm install & \
        cd frontend && npm install --legacy-peer-deps
        "
else
    echo "Usage: ./build.sh [prod|dev]"
fi