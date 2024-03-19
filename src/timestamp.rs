use chrono::{Datelike, NaiveDateTime, Timelike};
use nom::bytes::complete::{tag, take_till, take_while};
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::{AsChar, IResult};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Timestamp {
    // ? with that the inner value is private,
    // ? I don't want to the dependency on `chrono` pub
    pub(crate) inner: NaiveDateTime,
}

impl Timestamp {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        const DELIMITER_START: u8 = b'[';
        const DELIMITER_END: u8 = b']';

        let (input, (raw_date, raw_time)) = delimited(
            tag([DELIMITER_START]),
            separated_pair(
                context(
                    "date",
                    separated_list1(tag("."), take_while(|c| char::is_ascii_digit(&(c as char)))),
                ),
                tag(", "),
                context(
                    "time",
                    separated_list1(tag(":"), take_while(|c| char::is_ascii_digit(&(c as char)))),
                ),
            ),
            tag([DELIMITER_END]),
        )(input)?;

        // TODO remove this asserts, maybe there is way to check via nom if the parser was able to 'run' x amount of time
        assert!(raw_date.len() == 3);
        assert!(raw_time.len() == 3);

        let raw_date = raw_date
            .iter()
            .map(|raw| String::from_utf8_lossy(&raw))
            .collect::<Vec<_>>()
            .join(".");

        let raw_time = raw_time
            .iter()
            .map(|raw| String::from_utf8_lossy(&raw))
            .collect::<Vec<_>>()
            .join(":");

        let raw_datetime = format!("{raw_date}, {raw_time}");

        Ok((
            input,
            Timestamp {
                inner: NaiveDateTime::parse_from_str(&raw_datetime, "%d.%m.%y, %H:%M:%S")
                    .map_err(|err| {
                        eprintln!("input: {:?}", raw_datetime);
                        err
                    })
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

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::timestamp::Timestamp;
    use crate::SIMPLE_TEST_MESSAGE;

    #[test]
    fn just_works() {
        let (input, ts) = Timestamp::parse(SIMPLE_TEST_MESSAGE).unwrap();
        assert_eq!(input, b" LetsMelon: Hello World!");
        assert_eq!(
            ts,
            Timestamp {
                inner: NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                    NaiveTime::from_hms_opt(1, 2, 3).unwrap()
                )
            }
        )
    }
}
