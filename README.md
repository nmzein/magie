# Nemato

## About

Nemato is a gigapixel image & annotations rendering engine with a web interface for usage in many professional fields for the analysis of extremely large images.

## 💽 Installation

1. Download the source code: `git clone https://github.com/nmzein/nemato.git`
2. Navigate to main directory: `cd nemato`

---

### Docker (recommended)

1. Install [Docker Engine](https://docs.docker.com/engine/install/).
2. Ensure the Docker daemon is [running](https://docs.docker.com/config/daemon/start/).
3. Run the following commands (note that you may need to run them with `sudo`):

```
# 🛠️ Production [4.81GB; <6 mins on low-perf laptop]
docker-compose up prod

# 🏗️ Development [6.45GB; <3 mins on low-perf laptop]
docker-compose up dev
```

The application can now be accessed at `0.0.0.0:4000`.

#### Useful Docker Commands

Replace any instance of `<branch>` with `prod` or `dev`.

- Use `docker-compose up --build <branch>` to force a rebuild if any files are altered.
- Use `docker exec -it $(docker ps -qf "ancestor=nemato:<branch>") bash` to get a bash shell inside of the container.

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

sudo npm install -g vite
```

To run Nemato, navigate to `backend/rendering-engine/` and run:

```
# 🛠️ Production
cargo run --release

# 🏗️ Development
cargo run
```

Then, in another terminal, navigate to `frontend/` and run:

```
# 🛠️ Production
npm run prod -- --open

# 🏗️ Development
npm run dev -- --open
```

The application can now be accessed at `localhost:4000`.

---

### 🗃️ Interacting with the Database

To reset the database, run migrations, or prepare SQL queries, run:

```
cargo install sqlx-cli

# Create
cargo sqlx database create

# Migrate
cargo sqlx migrate run

# Prepare Queries
cargo sqlx prepare

# Drop Database
cargo sqlx database reset
```
