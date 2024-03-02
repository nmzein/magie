FROM node:21.6.2-bookworm-slim AS frontend

RUN --mount=type=cache,target=/var/cache/apt \
    apt-get update

RUN --mount=type=cache,target=/var/cache/npm \
    npm install -g npm@10.5.0 && \
    npm install -g vite

WORKDIR /app/frontend
COPY ./frontend /app/frontend
RUN npm install --legacy-peer-deps

## Release.
# RUN npm run build

FROM rust:slim-bookworm AS backend

RUN --mount=type=cache,target=/var/cache/apt \
    apt-get update
    
RUN --mount=type=cache,target=/var/cache/apt \
    apt-get install -y build-essential \
                       cmake \
                       nasm \
                       pkg-config \
                       libclang-dev \
                       libopenslide-dev \
                       libssl-dev

RUN --mount=type=cache,target=/var/cache/sqlx \
    cargo install sqlx-cli

## Development
# Copy only necessary files to minimise rebuild.
WORKDIR /app/backend/rendering-engine
COPY .env /app
COPY ./backend/rendering-engine/migrations /app/backend/rendering-engine/migrations
COPY ./backend/rendering-engine/.sqlx /app/backend/rendering-engine/.sqlx
COPY ./backend/rendering-engine/state /app/backend/rendering-engine/state
RUN cargo sqlx database create
RUN cargo sqlx migrate run

# Incremental build.
RUN echo "fn main() {}" > dummy.rs
COPY ./backend/rendering-engine/Cargo.toml /app/backend/rendering-engine/
COPY ./backend/rendering-engine/Cargo.lock /app/backend/rendering-engine/
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release

RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY ./backend/rendering-engine/src /app/backend/rendering-engine/src
RUN cargo build --release

COPY ./backend/store /app/backend/store

## Release.
# COPY ./backend/rendering-engine /app/backend/rendering-engine
# WORKDIR /app/backend/rendering-engine
# RUN cargo sqlx database create
# RUN cargo sqlx migrate run
# RUN cargo build --release

FROM rust:slim-bookworm AS final

COPY --from=frontend /usr/local/include/node /usr/local/include/node
COPY --from=frontend /usr/local/lib/node_modules /usr/local/lib/node_modules
COPY --from=frontend /usr/local/bin/node /usr/local/bin/node
RUN ln -s /usr/local/lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm

# TODO: Fix cache copying from backend.
COPY --from=backend . .

WORKDIR /app
COPY .env /app
# TODO: Don't copy full source code.
COPY --from=frontend /app/frontend /app/frontend
COPY --from=backend /app/backend /app/backend

EXPOSE 4000
EXPOSE 3000

## Development.
CMD /bin/bash -c "cd backend/rendering-engine && cargo run --release & cd frontend/ && npm run dev -- --host"

## Release.
# CMD /bin/bash -c "cd backend/rendering-engine/ && cargo run --release & cd frontend/ && npm run preview -- --host"