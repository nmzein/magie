# Nemato

## About

Nemato is a gigapixel image & annotations rendering engine with a web interface for usage in many professional fields for the analysis of extremely large images.

## 💽 Installation

1. Download the source code: `git clone https://github.com/nmzein/nemato.git`
2. Navigate to main directory: `cd nemato`

---

### Docker (recommended)

1. Install [Docker Desktop](https://docs.docker.com/desktop/) (needed for use of Docker Engine).
2. Ensure the docker daemon is [running](https://docs.docker.com/config/daemon/start/), and run:

```
docker build -t nemato .
docker run -p 3000:3000 -p 4000:4000 nemato

# Note you may need to run these with sudo.
```

The application can now be accessed at `0.0.0.0:4000`.

---

### Manual

Install dependencies (prerequisites: `curl`).

```
# Debian [verified working version(s): 12]
sudo apt install build-essential cmake nasm npm pkg-config libclang-dev libopenslide-dev libssl-dev

# Ubuntu
sudo apt install cmake nasm npm pkg-config libclang-dev libopenslide-dev libssl-dev
```

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
export PATH=$PATH:~/.cargo/bin
cargo install sqlx-cli

sudo npm install -g vite
```

### 🛠️ Production

To run Nemato in production, navigate to `backend/rendering-engine/` and run:

```
cargo sqlx database create
cargo sqlx migrate run
cargo run --release
```

Then, in another terminal, navigate to `frontend/` and run:

```
npm install --legacy-peer-deps
npm run build
npm run preview -- --open
```

### 🏗️ Development

To run Nemato for development, navigate to `backend/rendering-engine/` and run:

```
cargo sqlx database create
cargo sqlx migrate run
cargo run
```

Then, in another terminal, navigate to `frontend/` and run:

```
npm install --legacy-peer-deps
npm run dev -- --open
```
