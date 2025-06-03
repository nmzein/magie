<h1 align="center">
    MAGIE [alpha]
</h1>

MAGIE (Massively Annotated Gigapixel Image Explorer) is a web application and backend server for rendering and displaying multi-gigapixel images & millions of annotations.

<div style="display: flex;">
    <img width="49%" src="https://github.com/nmzein/magie/assets/67694622/694a43dd-fd48-416d-b036-fed7210d031f" alt="Demo Image 1" />
    <img width="49%" src="https://github.com/nmzein/magie/assets/67694622/102ab83d-ee10-4a21-b511-a598ac55cc50" alt="Demo Image 2" />
</div>

### Features

- ⚡️ **Extremely performant** Rust backend and Svelte frontend, utilising WebGL to render hundreds of thousands of annotations in milliseconds.

- 🖥️ **Sleek and modern** user interface.

- 🗂️ Remotely access your library using a **fully-featured file explorer**.

- 🛠️ **Easily integrate** your own image decoders, encoders, and analysis tools using a powerful, **flexible module interface**.

- 🌐 **Accessible anywhere at any time** using any modern web browser.

---

## 💽 Installation

1. Download the source code: `git clone https://github.com/nmzein/magie.git`
2. Navigate to main directory: `cd magie`

### Containerised

Install [Docker Engine](https://docs.docker.com/engine/install/) and ensure the [daemon is running](https://docs.docker.com/config/daemon/start/).

```
# Production [http://localhost:4000]
docker compose up prod

# Development [http://localhost:4000]
docker compose up --build dev
```

### Bare Metal

Install the [Nix package manager](https://nixos.org/download/).

```
# Production [http://localhost:3000]
nix run

# Development [http://localhost:4000]
nix develop
./dev.sh
```
