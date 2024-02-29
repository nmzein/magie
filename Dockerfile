FROM ubuntu:23.10

ENV DEBIAN_FRONTEND=noninteractive
WORKDIR /app
COPY ./.env /app

RUN --mount=type=cache,target=/var/cache/apt \
    apt-get update && \
    apt-get install -y nano

# Install frontend dependencies.
RUN --mount=type=cache,target=/var/cache/apt \
    apt-get install -y npm && \
    npm install -g vite

# Install backend dependencies.
RUN --mount=type=cache,target=/var/cache/apt \
    apt-get install -y libclang-dev \
                       libssl-dev \
                       libopenslide-dev \
                       pkg-config \
                       cmake \
                       nasm \
                       curl 

ENV ASM_NASM=/usr/bin/nasm

RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install sqlx-cli
RUN cargo install cargo-watch

# Build backend.
COPY ./backend/rendering-engine/migrations /app/backend/rendering-engine/migrations
COPY ./backend/rendering-engine/.sqlx /app/backend/rendering-engine/.sqlx
COPY ./backend/rendering-engine/state /app/backend/rendering-engine/state
RUN cd backend/rendering-engine/ && \
    cargo sqlx database create && \
    cargo sqlx migrate run

COPY ./backend/rendering-engine/src /app/backend/rendering-engine/src
COPY ./backend/rendering-engine/Cargo.toml /app/backend/rendering-engine
COPY ./backend/rendering-engine/Cargo.lock /app/backend/rendering-engine
RUN cd backend/rendering-engine/ && \
    cargo build

COPY ./backend/store /app/backend/store

# Build frontend.
COPY ./frontend /app/frontend

RUN cd frontend/ && \
    npm install --legacy-peer-deps

# Backend and frontend ports.
EXPOSE 3000
EXPOSE 4000

CMD /bin/bash -c "cd ./backend/rendering-engine/ && cargo run & cd ./frontend/ && npm run dev -- --host"