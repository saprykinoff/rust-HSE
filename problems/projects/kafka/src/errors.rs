#[derive(Debug)]
pub enum KafkaError {
    PortIsBusy,
    IoError(std::io::Error),
    JsonError(serde_json::error::Error),
}

impl From<std::io::Error> for KafkaError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<serde_json::error::Error> for KafkaError {
    fn from(value: serde_json::error::Error) -> Self {
        Self::JsonError(value)
    }
}