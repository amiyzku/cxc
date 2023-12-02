#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Websocket connection error: {0}")]
    WebsocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Websocket connection closed")]
    WebsocketConnectionClosed(),

    #[error("Json serialize error: {0}")]
    JsonSerializeError(serde_json::Error),

    #[error("Json deserialize error: {0}")]
    JsonDeserializeError(serde_json::Error),

    #[error("Subscribe error: {0}")]
    SubscribeError(String),

    #[error("Validation error: {0}")]
    ValidateError(#[from] garde::Report),
}
