# Nemato

## About

Nemato is a gigapixel image & annotations rendering engine with a web interface for usage in many professional fields for the analysis of extremely large images. 

## 💽 Installation

Install the correct dependencies for your operating system.

### Ubuntu
```
sudo apt install libclang-dev libssl-dev libopenslide-dev pkg-config npm
snap install rustup --classic
```
---
Install other dependencies.

```
sudo npm install -g vite
rustup default stable
cargo install sqlx-cli
```

## 🛠️ Build

To run Nemato in production, navigate to `backend/rendering-engine/` and run:

```
cargo sqlx database create
cargo sqlx migrate run
cargo sqlx prepare
cargo run --release
```

Then navigate to `frontend/` and run:

```
npm install
npm run build
npm run preview -- --open
```

## 🏗️ Development

To run Nemato for development, navigate to `backend/rendering-engine/` and run:

```
cargo sqlx database create
cargo sqlx migrate run
cargo sqlx prepare
cargo run
```

Then navigate to `frontend/` and run:

```
npm install
npm run dev -- --open
```


