use chrono::{Datelike, NaiveDateTime, Timelike};
use nom::bytes::complete::{tag, take_while};
use nom::sequence::delimited;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Timestamp {
    // ? with that the inner value is private,
    // ? I don't want to the dependency on `chrono` pub
    pub(crate) inner: NaiveDateTime,
}

impl Timestamp {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
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
