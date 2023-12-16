use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum KafkaError {
    PortIsBusy,
    IoError(std::io::Error),
    DeserializationError(serde_json::error::Error),
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

impl Display for KafkaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "I will definitely fix it later")
    }
}
