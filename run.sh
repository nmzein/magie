#!/bin/bash

if [ "$1" = "prod" ]; then
    eval "
        cd backend && cargo run --release & \
        cd frontend && bun run build && bun preview
        "
elif [ "$1" = "dev" ]; then
    eval "
        cd backend && cargo run & \
        cd frontend && bun dev
        "
else
    echo "Usage: ./run.sh [prod|dev]"
fi