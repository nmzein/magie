# Nemato

## About

Nemato is a gigapixel image & annotations rendering engine with a web interface for usage in many professional fields for the analysis of extremely large images.

## 💽 Installation

```
git clone https://github.com/nmzein/nemato.git
```

### Docker (recommended)

Navigate into `nemato/`, ensure the docker daemon is running, and run:

```
sudo docker build -t nemato .
sudo docker run -p 3000:3000 -p 4000:4000 nemato
```

### Manual

Install dependencies.

```
# Ubuntu
sudo apt install libclang-dev libssl-dev libopenslide-dev pkg-config npm
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
