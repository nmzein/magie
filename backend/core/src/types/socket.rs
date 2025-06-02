use crate::types::messages::ServerMsg;
use anyhow::Result;
use axum::extract::ws::Message;
use dashmap::DashMap;
use tokio::sync::{broadcast, mpsc};

type Broadcast = broadcast::Sender<Message>;
type Connections = DashMap<u32, mpsc::Sender<Message>>;

#[derive(Debug, Clone)]
pub struct ClientSocketManager {
    pub broadcast: Broadcast,
    connections: Connections,
}

impl Default for ClientSocketManager {
    fn default() -> Self {
        Self {
            broadcast: broadcast::channel(1024).0,
            connections: DashMap::new(),
        }
    }
}

impl ClientSocketManager {
    pub fn add_connection(&self, user_id: u32, sender: mpsc::Sender<Message>) {
        self.connections.insert(user_id, sender);
    }

    pub fn remove_connection(&self, user_id: u32) {
        self.connections.remove(&user_id);
    }

    // Send to specific user.
    pub async fn send(&self, user_id: u32, msg: ServerMsg) -> Result<()> {
        if let Some(sender) = self.connections.get(&user_id) {
            sender.send(msg.try_into()?).await?;
        }
        Ok(())
    }

    // Broadcast to all users.
    pub async fn broadcast(&self, msg: ServerMsg) -> Result<()> {
        self.broadcast.send(msg.try_into()?)?;
        Ok(())
    }
}
