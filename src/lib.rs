pub mod chat;
pub mod chat_participant;
pub mod message;
pub mod message_type;
pub mod timestamp;

#[cfg(test)]
pub(crate) const SIMPLE_TEST_MESSAGE: &[u8] = b"[01.02.24, 1:2:3] LetsMelon: Hello World!";
