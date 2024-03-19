use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1};
use nom::combinator::{map, opt};
use nom::sequence::terminated;
use nom::IResult;

use crate::chat_participant::ChatParticipant;
use crate::message_type::MessageType;
use crate::timestamp::Timestamp;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Message {
    pub(crate) timestamp: Timestamp,
    pub(crate) sender: ChatParticipant,
    pub(crate) message_type: MessageType,
}

impl Message {
    /// marker symbol UTF-32 `U+200E`
    pub(crate) const MARKER: [u8; 3] = [0xE2, 0x80, 0x8E];

    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, _image_or_document_marker) = opt(map(tag(Self::MARKER), |_| ()))(input)?;

        let (input, timestamp) = terminated(Timestamp::parse, space1)(input)?;
        let (input, sender) = terminated(ChatParticipant::parse, tag(": "))(input)?;
        let (input, message_type) = terminated(MessageType::parse, opt(line_ending))(input)?;

        // TODO return error with an better error message
        // assert!(
        //     (matches!(message_type, MessageType::Image)
        //         || matches!(message_type, MessageType::Document(_)))
        //         && image_or_document_marker.is_some(), "The parsed parsed the line as 'MessageType::Image' or 'MessageType::Document' although the marker at the beginning of the line was missing"
        // );

        Ok((
            input,
            Message {
                timestamp,
                sender,
                message_type,
            },
        ))
    }

    /// Returns the timestamp as `Timestamp`
    pub fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }

    /// Returns the sender as `ChatParticipant`
    pub fn sender(&self) -> &ChatParticipant {
        &self.sender
    }

    /// Returns the message data as `MessageType`
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::chat_participant::ChatParticipant;
    use crate::message::Message;
    use crate::message_type::MessageType;
    use crate::timestamp::Timestamp;
    use crate::SIMPLE_TEST_MESSAGE;

    #[test]
    fn parse_one_line() {
        let (input, message) = Message::parse(SIMPLE_TEST_MESSAGE).unwrap();
        assert_eq!(input, b"");
        assert_eq!(
            message,
            Message {
                timestamp: Timestamp {
                    inner: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                        NaiveTime::from_hms_opt(1, 2, 3).unwrap()
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
    fn parse_multiple_lines() {
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
                        NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                        NaiveTime::from_hms_opt(1, 2, 3).unwrap()
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
                        NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                        NaiveTime::from_hms_opt(1, 2, 3).unwrap()
                    )
                },
                sender: ChatParticipant {
                    name: "LetsMelon".to_string()
                },
                message_type: MessageType::Text("Hello World!".to_string())
            }
        );
    }
}
