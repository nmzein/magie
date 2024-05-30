# Nemato

## About

Nemato is a gigapixel image & annotations rendering engine, built in Rust, with a modern Svelte web interface. It enables scientists to view and analyse extremely large images annotated with hundreds of thousands of annotations created by automated tools. It also allows analysis tool developers to easily integrate their tools through a powerful and flexible module interface.

<p align="center">
  <img src="https://github.com/nmzein/nemato/assets/67694622/401968c1-dee3-4080-8634-41fd91aaf4d6" alt="Demo Image" width="700" />
</p>

## 💽 Installation

1. Download the source code: `git clone https://github.com/nmzein/nemato.git`
2. Navigate to main directory: `cd nemato`

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

| OS     | Command               | Verified Supported Version(s) | Issues                                     |
| ------ | --------------------- | ----------------------------- | ------------------------------------------ |
| Debian | `./install.sh debian` | 12/Bookworm                   | -                                          |
| Fedora | `./install.sh fedora` | -                             | https://github.com/nmzein/nemato/issues/13 |
| Ubuntu | `./install.sh ubuntu` | -                             | https://github.com/nmzein/nemato/issues/13 |

Build and run:

```
🛠️ Production
./run.sh prod

🏗️ Development
./run.sh dev
```

The application can now be accessed at `http://localhost:4000`.
