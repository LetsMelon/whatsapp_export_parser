pub mod chat;
pub mod chat_participant;
pub mod message;
pub mod message_type;
pub mod timestamp;

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::chat_participant::ChatParticipant;
    use crate::message::Message;
    use crate::message_type::MessageType;
    use crate::timestamp::Timestamp;

    const SIMPLE_TEST_MESSAGE: &[u8] = b"[02.10.23, 22:32:30] LetsMelon: Hello World!";

    #[test]
    fn chat_participant_parse() {
        let (input, chat_participant) = ChatParticipant::parse(b"LetsMelon: Hello World!").unwrap();
        assert_eq!(input, b": Hello World!");
        assert_eq!(
            chat_participant,
            ChatParticipant {
                name: "LetsMelon".to_string()
            }
        );
    }

    #[test]
    fn message_parse() {
        let (input, message) = Message::parse(SIMPLE_TEST_MESSAGE).unwrap();
        assert_eq!(input, b"");
        assert_eq!(
            message,
            Message {
                timestamp: Timestamp {
                    inner: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(),
                        NaiveTime::from_hms_opt(22, 32, 30).unwrap()
                    )
                },
                sender: ChatParticipant {
                    name: "LetsMelon".to_string()
                },
                message_type: MessageType::Text("Hello World!".to_string())
            }
        );
    }

    #[test]
    fn message_parse_lines() {
        let mut buffer = [0; SIMPLE_TEST_MESSAGE.len() * 2 + 1];
        buffer[..SIMPLE_TEST_MESSAGE.len()].copy_from_slice(SIMPLE_TEST_MESSAGE);
        buffer[SIMPLE_TEST_MESSAGE.len()] = b'\n';
        buffer[(SIMPLE_TEST_MESSAGE.len() + 1)..].copy_from_slice(SIMPLE_TEST_MESSAGE);

        let (input, message) = Message::parse(&buffer).unwrap();
        assert_eq!(
            message,
            Message {
                timestamp: Timestamp {
                    inner: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(),
                        NaiveTime::from_hms_opt(22, 32, 30).unwrap()
                    )
                },
                sender: ChatParticipant {
                    name: "LetsMelon".to_string()
                },
                message_type: MessageType::Text("Hello World!".to_string())
            }
        );

        let (input, message) = Message::parse(&input).unwrap();
        assert_eq!(input, b"");
        assert_eq!(
            message,
            Message {
                timestamp: Timestamp {
                    inner: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(),
                        NaiveTime::from_hms_opt(22, 32, 30).unwrap()
                    )
                },
                sender: ChatParticipant {
                    name: "LetsMelon".to_string()
                },
                message_type: MessageType::Text("Hello World!".to_string())
            }
        );
    }

    #[test]
    fn timestamp_parse() {
        let (input, ts) = Timestamp::parse(SIMPLE_TEST_MESSAGE).unwrap();
        assert_eq!(input, b" LetsMelon: Hello World!");
        assert_eq!(
            ts,
            Timestamp {
                inner: NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(),
                    NaiveTime::from_hms_opt(22, 32, 30).unwrap()
                )
            }
        )
    }
}
