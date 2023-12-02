use tokio_tungstenite::tungstenite::Message;

use crate::{error::AppError, websocket_base::WebsocketBase};

pub struct Websocket {
    pub base: WebsocketBase,
}

impl Websocket {
    pub async fn connect(channel: String) -> Result<Self, AppError> {
        let ws = WebsocketBase::connect(channel).await?;
        Ok(Self { base: ws })
    }

    pub async fn subscribe(&mut self, topics: &Vec<String>) -> Result<(), AppError> {
        unimplemented!("Use connect() instead")
    }

    pub async fn unsubscribe(&mut self, topics: &Vec<String>) -> Result<(), AppError> {
        unimplemented!("Unused")
    }

    pub async fn auth(&mut self) -> Result<(), AppError> {
        unimplemented!("Unused")
    }

    pub async fn pong(&mut self) -> Result<(), AppError> {
        self.base.write(Message::Pong(vec![])).await?;
        Ok(())
    }
}
