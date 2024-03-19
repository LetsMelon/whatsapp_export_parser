use std::io::{BufRead, BufReader, Read};

use nom::multi::many1;
use nom::IResult;

use crate::message::Message;

#[derive(Debug)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    // TODO proper error handling
    pub fn parse(input: &[u8]) -> Result<Self, ()> {
        fn parse_internal(input: &[u8]) -> IResult<&[u8], Chat> {
            let (input, messages) = many1(Message::parse)(input)?;

            Ok((input, Chat { messages }))
        }

        match parse_internal(input) {
            Ok((input, chat)) => {
                if input.len() == 0 {
                    Ok(chat)
                } else {
                    eprintln!(
                        "The input.len() should be zero, couldn't parsed: {:?}",
                        String::from_utf8_lossy(input)
                    );
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
    pub fn parse_from_reader<R: Read>(reader: R) -> Result<Self, ()> {
        let reader = BufReader::new(reader);

        let mut messages = Vec::new();

        for line in reader.lines() {
            // TODO proper error handling
            let line = line.map_err(|_| ())?;

            let (input, message) = Message::parse(line.as_bytes()).map_err(|err| {
                eprintln!("encountered an error while parsing: {err:?}");
                ()
            })?;

            if !input.is_empty() {
                eprintln!(
                    "The input.len() should be zero, couldn't parsed: {:?}",
                    String::from_utf8_lossy(input)
                );
                return Err(());
            }

            messages.push(message);
        }

        Ok(Chat { messages })
    }

    /// Get all messages as `Message` of the `Chat`
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }
}
