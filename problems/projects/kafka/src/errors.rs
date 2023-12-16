#[derive(Debug)]
pub enum KafkaError {
    PortIsBusy,
    IoError(std::io::Error),
    DeserializationError(serde_json::error::Error),
    JsonParseError(String),
    RegistrationIsRequired
}

impl From<std::io::Error> for KafkaError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<serde_json::error::Error> for KafkaError {
    fn from(value: serde_json::error::Error) -> Self {
        Self::DeserializationError(value)
    }
}