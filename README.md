# MAGIE [beta]

MAGIE (Massively Annotated Gigapixel Image Explorer) is a web application and backend server for rendering and displaying multi-gigapixel images & millions of annotations. Because the entire stack is extremely lean and built from scratch, the performance acheived is unparalleled compared to other similar tools. MAGIE is built with modularity in mind, allowing developers to add support for their own image decoders, analysis tools, and more.

Currently, the application is in beta (so some bugs have not yet been fixed) however, it is already stable enough for you to test if you would like.

- ⚡️ Extremely performant Rust backend and Svelte frontend, utilising WebGL to render hundreds of thousands of annotations in seconds.
- 🖥️ Sleek and modern user interface.
- 🗂️ Navigate your stored files using a highly-functional file explorer.
- 🛠️ Easily integrate your own decoders and analysis tools using a powerful, flexible module interface.
- 🌐 Remotely access your library using your browser.
- 🎨 Change the colour, opacity, and visibility of your annotations with instant millisecond rerenders.

<div style="display: flex;">
    <img width="49%" src="https://github.com/nmzein/magie/assets/67694622/694a43dd-fd48-416d-b036-fed7210d031f" alt="Demo Image 1" />
    <img width="49%" src="https://github.com/nmzein/magie/assets/67694622/102ab83d-ee10-4a21-b511-a598ac55cc50" alt="Demo Image 2" />
</div>

## 💽 Installation

1. Download the source code: `git clone https://github.com/nmzein/magie.git`
2. Navigate to main directory: `cd magie`

---

### Docker (recommended)

1. Install [Docker Engine](https://docs.docker.com/engine/install/).
2. Ensure the Docker daemon is [running](https://docs.docker.com/config/daemon/start/).
3. Run the following commands (note that you may need to run them with `sudo`):

```
🛠️ Production [4.09GB; <6 mins on low-perf laptop]
docker-compose up prod

🏗️ Development [6.45GB; <3 mins on low-perf laptop]
docker-compose up --build dev
```

The application can now be accessed at `http://0.0.0.0:4000`.

---

### Bare Metal

Install dependencies:

| OS     | Command               | Verified Supported Version(s) | Issues                                    |
| ------ | --------------------- | ----------------------------- | ----------------------------------------- |
| Debian | `./install.sh debian` | 12/Bookworm                   | -                                         |
| Fedora | `./install.sh fedora` | -                             | https://github.com/nmzein/magie/issues/13 |
| Ubuntu | `./install.sh ubuntu` | -                             | https://github.com/nmzein/magie/issues/13 |

Build and run:

```
🛠️ Production
./build.sh prod
./run.sh prod

🏗️ Development
./build.sh dev
./run.sh dev
```

The application can now be accessed at `http://localhost:4000`.
