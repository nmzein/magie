use crate::types::{messages::{GeneralServerMsg, AssetServerMsg}, user::UserId};
use anyhow::Result;
use axum::extract::ws::Message;
use dashmap::DashMap;
use tokio::sync::{broadcast, mpsc};

type Broadcast = broadcast::Sender<Message>;
type Tx = mpsc::Sender<Message>;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct AssetKey {
    pub store_id: u32,
    pub asset_id: u32,
}

type GeneralConnections = DashMap<UserId, Tx>;

type AssetConnections = DashMap<UserId, DashMap<AssetKey, Tx>>;

#[derive(Debug, Clone)]
pub struct ClientSocketManager {
    pub broadcast: Broadcast,
    general: GeneralConnections,
    assets: AssetConnections,
}

impl Default for ClientSocketManager {
    fn default() -> Self {
        Self {
            broadcast: broadcast::channel(1024).0,
            general: DashMap::new(),
            assets: DashMap::new(),
        }
    }
}

impl ClientSocketManager {
    /// Register the client's general connection
    pub fn add_general_connection(&self, user_id: UserId, sender: Tx) {
        self.general.insert(user_id, sender);
    }

    /// Register a (store_id, asset_id) connection for a client
    pub fn add_asset_connection(
        &self,
        user_id: UserId,
        store_id: u32,
        asset_id: u32,
        sender: Tx,
    ) {
        let key = AssetKey { store_id, asset_id };

        self.assets
            .entry(user_id)
            .or_insert_with(DashMap::new)
            .insert(key, sender);
    }

    /// Remove the general connection and ALL asset connections
    pub fn remove_all_connections(&self, user_id: UserId) {
        self.general.remove(&user_id);
        self.assets.remove(&user_id);
    }

    /// Remove a single asset connection
    pub fn remove_asset_connection(
        &self,
        user_id: UserId,
        store_id: u32,
        asset_id: u32,
    ) {
        if let Some(user_assets) = self.assets.get(&user_id) {
            user_assets.remove(&AssetKey { store_id, asset_id });
        }
    }

    /// Send to the client's general connection
    pub async fn send_general(
        &self,
        user_id: UserId,
        msg: GeneralServerMsg,
    ) -> Result<()> {
        if let Some(sender) = self.general.get(&user_id) {
            sender.send(msg.try_into()?).await?;
        }
        Ok(())
    }

    /// Send to a specific (store_id, asset_id) connection
    pub async fn send_asset(
        &self,
        user_id: UserId,
        store_id: u32,
        asset_id: u32,
        msg: AssetServerMsg,
    ) -> Result<()> {
        if let Some(user_assets) = self.assets.get(&user_id) {
            if let Some(sender) =
                user_assets.get(&AssetKey { store_id, asset_id })
            {
                sender.send(msg.try_into()?).await?;
            }
        }
        Ok(())
    }

    /// Broadcast to all connected sockets
    pub async fn broadcast(&self, msg: GeneralServerMsg) -> Result<()> {
        self.broadcast.send(msg.try_into()?)?;
        Ok(())
    }
}
