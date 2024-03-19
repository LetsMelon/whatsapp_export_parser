use chrono::NaiveDateTime;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::space1;
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{delimited, terminated};
use nom::IResult;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ChatParticipant {
    name: String,
}

impl ChatParticipant {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(take_until(":"), |item: &str| ChatParticipant {
            name: item.to_string(),
        })(input)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Timestamp {
    // ? with that the inner value is private,
    // ? I don't want to the dependency on `chrono` pub
    inner: NaiveDateTime,
}

impl Timestamp {
    fn parse(input: &str) -> IResult<&str, Self> {
        const DELIMITER_START: char = '[';
        const DELIMITER_END: char = ']';

        let (input, ts_raw) = delimited(
            tag(DELIMITER_START.to_string().as_str()), // TODO maybe remove the `.to_string()` call
            take_while(|item| item != DELIMITER_END),
            tag(DELIMITER_END.to_string().as_str()), // TODO maybe remove the `.to_string()` call
        )(input)?;

        Ok((
            input,
            Timestamp {
                inner: NaiveDateTime::parse_from_str(ts_raw, "%d.%m.%y, %H:%M:%S").unwrap(), // TODO return proper error
            },
        ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum MessageType {
    Text(String),
    Image,
    Document(String),
    InternalMessage(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Message {
    timestamp: Timestamp,
    sender: ChatParticipant,
    message_type: MessageType,
}

impl Message {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, timestamp) = terminated(Timestamp::parse, space1)(input)?;
        let (input, sender) = terminated(ChatParticipant::parse, tag(": "))(input)?;
        // TODO parse
        let (input, msg) = terminated(take_while(|item| item != '\n'), opt(tag("\n")))(input)?;

        Ok((
            input,
            Message {
                timestamp,
                sender,
                message_type: MessageType::Text(msg.to_string()),
            },
        ))
    }
}

#[derive(Debug)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    fn parse_internal(input: &str) -> IResult<&str, Self> {
        let (input, messages) = many1(Message::parse)(input)?;

        Ok((input, Chat { messages }))
    }

    // TODO proper error handling
    pub fn parse(input: &str) -> Result<Self, ()> {
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
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::{ChatParticipant, Message, MessageType, Timestamp};

    const SIMPLE_TEST_MESSAGE: &str = "[02.10.23, 22:32:30] LetsMelon: Hello World!";

    #[test]
    fn chat_participant_parse() {
        let (input, chat_participant) = ChatParticipant::parse("LetsMelon: Hello World!").unwrap();
        assert_eq!(input, ": Hello World!");
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
        assert_eq!(input, "");
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
        let input = format!("{}\n{}", SIMPLE_TEST_MESSAGE, SIMPLE_TEST_MESSAGE);

        let (input, message) = Message::parse(&input).unwrap();
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
        assert_eq!(input, "");
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
        assert_eq!(input, " LetsMelon: Hello World!");
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
