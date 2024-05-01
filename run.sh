#!/bin/bash

if [ "$1" = "prod" ]; then
    eval "
        cd backend && cargo run --release & \
        cd frontend && npm run prod
        "
elif [ "$1" = "dev" ]; then
    eval "
        cd backend && cargo run & \
        cd frontend && npm run dev
        "
else
    echo "Usage: ./run.sh [prod|dev]"
fi