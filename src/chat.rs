use std::io::Read;

use nom::multi::many1;
use nom::IResult;

use crate::message::Message;

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
