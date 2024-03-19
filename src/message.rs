use nom::bytes::complete::tag;
use nom::character::complete::space1;
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
    // marker symbol 'U+200E'
    pub(crate) const MARKER: [u8; 3] = [0xE2, 0x80, 0x8E];

    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, _image_or_document_marker) = opt(map(tag(Self::MARKER), |_| ()))(input)?;

        let (input, timestamp) = terminated(Timestamp::parse, space1)(input)?;
        let (input, sender) = terminated(ChatParticipant::parse, tag(": "))(input)?;
        let (input, message_type) = terminated(MessageType::parse, opt(tag("\n")))(input)?;

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
