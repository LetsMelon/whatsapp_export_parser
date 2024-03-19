use std::io::Read;

use chrono::{Datelike, NaiveDateTime, Timelike};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{space0, space1};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ChatParticipant {
    name: String,
}

impl ChatParticipant {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(take_until(":"), |raw: &[u8]| ChatParticipant {
            name: String::from_utf8(raw.to_vec()).unwrap(),
        })(input)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Timestamp {
    // ? with that the inner value is private,
    // ? I don't want to the dependency on `chrono` pub
    inner: NaiveDateTime,
}

impl Timestamp {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        const DELIMITER_START: char = '[';
        const DELIMITER_END: char = ']';

        let (input, ts_raw) = delimited(
            tag(DELIMITER_START.to_string().as_str()), // TODO maybe remove the `.to_string()` call
            take_while(|item| item != DELIMITER_END as u8),
            tag(DELIMITER_END.to_string().as_str()), // TODO maybe remove the `.to_string()` call
        )(input)?;

        Ok((
            input,
            Timestamp {
                inner: NaiveDateTime::parse_from_str(
                    // TODO remove to vec
                    &String::from_utf8(ts_raw.to_vec()).unwrap(),
                    "%d.%m.%y, %H:%M:%S",
                )
                .unwrap(), // TODO return proper error
            },
        ))
    }

    /// Get the year, month and day of the `Timestamp`
    ///
    /// Month and day starts at 1.
    pub fn ymd(&self) -> (i32, u32, u32) {
        let date = self.inner.date();

        let year = date.year();
        let month = date.month0() + 1;
        let day = date.day0() + 1;

        (year, month, day)
    }

    /// Get the hours, minutes and seconds of the `Timestamp`
    pub fn hms(&self) -> (u32, u32, u32) {
        let time = self.inner.time();

        let hour = time.hour();
        let minutes = time.minute();
        let seconds = time.second();

        (hour, minutes, seconds)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum MessageType {
    Text(String),
    Image,
    Document(String),
    InternalMessage(String),
    Location(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Message {
    timestamp: Timestamp,
    sender: ChatParticipant,
    message_type: MessageType,
}

impl Message {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        // marker symbol 'U+200E'
        const MARKER: [u8; 3] = [0xE2, 0x80, 0x8E];

        let (input, image_or_document_marker) = opt(map(tag(MARKER), |_| ()))(input)?;

        dbg!(image_or_document_marker);

        let (input, timestamp) = terminated(Timestamp::parse, space1)(input)?;
        let (input, sender) = terminated(ChatParticipant::parse, tag(": "))(input)?;

        let (input, message_type) = terminated(
            alt((
                map(
                    separated_pair(
                        take_while(|item| item != b' '),
                        tuple((space0, map(tag(MARKER), |_: &[u8]| ()))),
                        tag("document omitted"),
                    ),
                    |(items, _)| MessageType::Document(String::from_utf8(items.to_vec()).unwrap()),
                ),
                map(
                    tuple((
                        space0,
                        map(tag(MARKER), |_: &[u8]| ()),
                        tag("image omitted"),
                    )),
                    |_| MessageType::Image,
                ),
                map(
                    preceded(
                        tuple((space0, map(tag(MARKER), |_: &[u8]| ()), tag("Location: "))),
                        take_while(|c| c != b'\n'),
                    ),
                    |raw_location: &[u8]| {
                        MessageType::Location(String::from_utf8(raw_location.to_vec()).unwrap())
                    },
                ),
                map(
                    preceded(
                        tuple((space0, map(tag(MARKER), |_: &[u8]| ()))),
                        take_while(|c| c != b'\n'),
                    ),
                    |raw: &[u8]| {
                        MessageType::InternalMessage(String::from_utf8(raw.to_vec()).unwrap())
                    },
                ),
                map(take_while(|c| c != b'\n'), |raw_text: &[u8]| {
                    MessageType::Text(String::from_utf8(raw_text.to_vec()).unwrap())
                }),
            )),
            opt(tag("\n")),
        )(input)?;

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

#[derive(Debug)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    fn parse_internal(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, messages) = many1(Message::parse)(input)?;

        Ok((input, Chat { messages }))
    }

    // TODO proper error handling
    pub fn parse(input: &[u8]) -> Result<Self, ()> {
        match Self::parse_internal(input) {
            Ok((input, chat)) => {
                if input.len() == 0 {
                    Ok(chat)
                } else {
                    eprintln!("The input.len() should be zero, couldn't parsed: {input:?}");
                    Err(())
                }
            }
            Err(err) => {
                eprintln!("encountered an error while parsing: {err:?}");
                Err(())
            }
        }
    }

    // TODO proper error handling
    pub fn parse_from_reader<R: Read>(mut reader: R) -> Result<Self, ()> {
        // TODO use a way where the whole reader doesn't need to be read into memory before parsing

        let mut raw = Vec::with_capacity(1024);
        // TODO proper error handling
        reader.read_to_end(&mut raw).map_err(|_| ())?;

        Self::parse(&raw)
    }

    /// Get all messages as `Message` of the `Chat`
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::{ChatParticipant, Message, MessageType, Timestamp};

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
