use nom::bytes::complete::take_until;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
