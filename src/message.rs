use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{space0, space1};
use nom::combinator::{map, opt};
use nom::sequence::{preceded, separated_pair, terminated, tuple};
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
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
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
