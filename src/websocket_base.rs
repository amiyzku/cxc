use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::error::AppError;

pub struct WebsocketBase {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebsocketBase {
    pub async fn connect(channel: String) -> Result<Self, AppError> {
        let (stream, _) = connect_async(channel).await?;
        Ok(WebsocketBase { stream })
    }

    pub async fn read(&mut self) -> Result<Message, AppError> {
        match self.stream.next().await {
            Some(msg) => Ok(msg?),
            None => Err(AppError::WebsocketConnectionClosed()),
        }
    }

    pub async fn write(&mut self, msg: &str) -> Result<(), AppError> {
        let msg = Message::Text(msg.to_string());
        if let Ok(()) = self.stream.send(msg).await {
            Ok(())
        } else {
            Err(AppError::WebsocketConnectionClosed())
        }
    }
}
