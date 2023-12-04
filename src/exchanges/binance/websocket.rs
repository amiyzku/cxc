use tokio_tungstenite::tungstenite::Message;

use crate::{error::CxcError, websocket_base::WebsocketBase};

pub struct Websocket {
    pub base: WebsocketBase,
}

impl Websocket {
    pub async fn connect(channel: String) -> Result<Self, CxcError> {
        let ws = WebsocketBase::connect(channel).await?;
        Ok(Self { base: ws })
    }

    pub async fn subscribe(&mut self) -> Result<(), CxcError> {
        unimplemented!("Use connect() instead")
    }

    pub async fn unsubscribe(&mut self) -> Result<(), CxcError> {
        unimplemented!("Unused")
    }

    pub async fn auth(&mut self) -> Result<(), CxcError> {
        unimplemented!("Unused")
    }

    pub async fn pong(&mut self) -> Result<(), CxcError> {
        self.base.write(Message::Pong(vec![])).await?;
        Ok(())
    }
}
