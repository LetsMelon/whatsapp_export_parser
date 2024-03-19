use nom::bytes::complete::take_until;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatParticipant {
    pub(crate) name: String,
}

impl ChatParticipant {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(take_until(":"), |raw: &[u8]| ChatParticipant {
            name: String::from_utf8(raw.to_vec()).unwrap(),
        })(input)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::chat_participant::ChatParticipant;

    #[test]
    fn just_works() {
        let (input, chat_participant) = ChatParticipant::parse(b"LetsMelon: Hello World!").unwrap();
        assert_eq!(input, b": Hello World!");
        assert_eq!(
            chat_participant,
            ChatParticipant {
                name: "LetsMelon".to_string()
            }
        );
    }
}
