use axum::{
    extract::{WebSocketUpgrade, ws::Message},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;

pub async fn websocket(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| async {
        let (mut sink, mut stream) = socket.split();
        // Credit: https://gist.github.com/hexcowboy/8ebcf13a5d3b681aa6c684ad51dd6e0c
        // Create an mpsc channel so we can send messages to the sink from multiple threads.
        let (sender, mut receiver) = mpsc::channel::<Message>(4);

        // Spawn a task that forwards messages from the mpsc receiver to the websocket sink.
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                if sink.send(message).await.is_err() {
                    break;
                }
            }
        });

        while let Some(Ok(Message::Text(message))) = stream.next().await {
            let sender = sender.clone();

            tokio::spawn(async move {
                // TODO: Check message type and call relevant functions.
                crate::api::image::tiles::tiles(message, sender).await;
            });
        }
    })
}
