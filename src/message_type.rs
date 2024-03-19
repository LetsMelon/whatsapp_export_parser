use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;

use crate::message::Message;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum MessageType {
    Text(String),
    Image,
    Document(String),
    InternalMessage(String),
    Location(String),
}

impl MessageType {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            map(
                separated_pair(
                    take_while(|item| item != b' '),
                    tuple((space0, map(tag(Message::MARKER), |_: &[u8]| ()))),
                    tag("document omitted"),
                ),
                |(items, _)| MessageType::Document(String::from_utf8(items.to_vec()).unwrap()),
            ),
            map(
                tuple((
                    space0,
                    map(tag(Message::MARKER), |_: &[u8]| ()),
                    tag("image omitted"),
                )),
                |_| MessageType::Image,
            ),
            map(
                preceded(
                    tuple((
                        space0,
                        map(tag(Message::MARKER), |_: &[u8]| ()),
                        tag("Location: "),
                    )),
                    take_while(|c| c != b'\n'),
                ),
                |raw_location: &[u8]| {
                    MessageType::Location(String::from_utf8(raw_location.to_vec()).unwrap())
                },
            ),
            map(
                preceded(
                    tuple((space0, map(tag(Message::MARKER), |_: &[u8]| ()))),
                    take_while(|c| c != b'\n'),
                ),
                |raw: &[u8]| MessageType::InternalMessage(String::from_utf8(raw.to_vec()).unwrap()),
            ),
            map(take_while(|c| c != b'\n'), |raw_text: &[u8]| {
                MessageType::Text(String::from_utf8(raw_text.to_vec()).unwrap())
            }),
        ))(input)
    }
}
