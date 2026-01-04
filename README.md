<h1 align="center">
    MAGIE [alpha]
</h1>

MAGIE (Massively Annotated Gigapixel Image Explorer) is a web application and backend server for rendering and displaying multi-gigapixel images & millions of annotations.

<div style="display: flex;">
    <img width="49%" src="https://github.com/nmzein/magie/assets/67694622/694a43dd-fd48-416d-b036-fed7210d031f" alt="Demo Image 1" />
    <img width="49%" src="https://github.com/nmzein/magie/assets/67694622/102ab83d-ee10-4a21-b511-a598ac55cc50" alt="Demo Image 2" />
</div>

### Features

- **Extremely performant** Rust backend and Svelte frontend, utilising WebGL to render hundreds of thousands of annotations in milliseconds.

- **Sleek and modern** user interface.

- Remotely access your library using a **fully-featured file explorer**.

- **Easily integrate** your own image decoders, encoders, and analysis tools using a **flexible plugin API**.

- **Accessible anywhere at any time** using any modern web browser.

---

## 💽 Install

Install the [Nix package manager](https://nixos.org/download/).

### Bare Metal
```
nix run
```

### Container
```
nix run .#podman
# or
nix run .#docker
```

The app is now accessible at http://localhost:3000.

---

## 🏗️ Develop

Install the [Nix package manager](https://nixos.org/download/).

```
nix develop
nix run .#dev
```

The app is now accessible at http://localhost:4000.
