#!/usr/bin/env bash

cd backend && cargo run & \
cd backend/geometry-computer && bun install & \
cd frontend && bun install && bun run dev
