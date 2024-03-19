use std::io::{BufRead, BufReader, Read};

use either::Either;
use nom::multi::many1;
use nom::IResult;

use crate::message::Message;
use crate::message_type::MessageType;

#[derive(Debug)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    // TODO proper error handling
    pub fn parse(input: &[u8]) -> Result<Self, ()> {
        fn parse_internal(input: &[u8]) -> IResult<&[u8], Chat> {
            let (input, messages) = many1(Message::parse)(input)?;

            // TODO refactor into separate function
            let messages = messages.iter().fold(Vec::new(), |mut acc, item| {
                match item {
                    Either::Left(msg) => acc.push(msg.clone()),
                    Either::Right(text) => {
                        let msg = acc.last_mut() .expect( &format!("Error in parsing. This error can only happen if no line has been parsed as an message.\ntext:{text:?}") );
                        match &mut msg.message_type {
                            MessageType::Text(old_text) => {
                                old_text.push_str("\n");
                                old_text.push_str(&text);
                            }
                            _ => panic!(
                                "A new line for a text is only supported if the message is a text."
                            ),
                        };
                    }
                };
    
                acc
            });

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
                let err = err.map(|item| (item.code, String::from_utf8_lossy(item.input)));

                eprintln!("encountered an error while parsing: {:?}", err);
                eprintln!("Line: {:#04X?}", line.as_bytes());

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

        let messages = messages.iter().fold(Vec::new(), |mut acc, item| {
            match item {
                Either::Left(msg) => acc.push(msg.clone()),
                Either::Right(text) => {
                    let msg = acc.last_mut() .expect( &format!("Error in parsing. This error can only happen if no line has been parsed as an message.\ntext:{text:?}") );
                    match &mut msg.message_type {
                        MessageType::Text(old_text) => {
                            old_text.push_str("\n");
                            old_text.push_str(&text);
                        }
                        _ => panic!(
                            "A new line for a text is only supported if the message is a text."
                        ),
                    };
                }
            };

            acc
        });

        Ok(Chat { messages })
    }

    /// Get all messages as `Message` of the `Chat`
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }
}
