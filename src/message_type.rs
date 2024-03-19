#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum MessageType {
    Text(String),
    Image,
    Document(String),
    InternalMessage(String),
    Location(String),
}
