use tokio_tungstenite::tungstenite::Message;

use crate::{error::CxcError, websocket_base::WebsocketBase};

use super::request_params::{Event, Subscribe};

pub struct Websocket {
    pub base: WebsocketBase,
}

impl Websocket {
    pub async fn connect(channel: String) -> Result<Self, CxcError> {
        let ws = WebsocketBase::connect(channel).await?;
        Ok(Self { base: ws })
    }

    pub async fn subscribe(&mut self, subscription: Subscribe) -> Result<(), CxcError> {
        let json =
            serde_json::to_string(&subscription).map_err(|e| CxcError::JsonSerializeError(e))?;
        self.base.write(Message::Text(json)).await?;
        Ok(())
    }

    pub async fn ping(&mut self) -> Result<(), CxcError> {
        let event = Event::ping();
        let json = serde_json::to_string(&event).map_err(|e| CxcError::JsonSerializeError(e))?;
        println!("{}", json);
        self.base.write(Message::Text(json)).await?;
        Ok(())
    }
}
