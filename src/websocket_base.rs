use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::error::CxcError;

pub struct WebsocketBase {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebsocketBase {
    pub async fn connect(channel: String) -> Result<Self, CxcError> {
        let (stream, _) = connect_async(channel).await?;
        Ok(WebsocketBase { stream })
    }

    pub async fn read(&mut self) -> Result<Message, CxcError> {
        match self.stream.next().await {
            Some(msg) => Ok(msg?),
            None => Err(CxcError::WebsocketConnectionClosed()),
        }
    }

    pub async fn write(&mut self, msg: Message) -> Result<(), CxcError> {
        if let Ok(()) = self.stream.send(msg).await {
            Ok(())
        } else {
            Err(CxcError::WebsocketConnectionClosed())
        }
    }
}
