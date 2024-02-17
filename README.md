Install dependencies (Ubuntu):  
`sudo apt install libclang-dev libssl-dev libopenslide-dev pkg-config npm`  
`snap install rustup --classic`  
`npm install -g vite`  
`rustup default stable`

In /backend/rendering-engine:  
`cargo install sqlx-cli`  
`cargo build`  
`cargo sqlx database create`  
`cargo sqlx migrate run`  
`cargo sqlx prepare`  
`cargo run`

In /frontend:  
`npm install`  
`npm run dev -- --open`
