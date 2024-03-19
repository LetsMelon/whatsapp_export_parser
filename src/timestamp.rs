use chrono::{Datelike, NaiveDateTime, Timelike};
use nom::bytes::complete::{tag, take_while};
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Timestamp {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

impl Timestamp {
    pub(crate) fn from_naive_date_time(naive_date_time: NaiveDateTime) -> Self {
        let date = naive_date_time.date();
        let year = date.year();
        let month = date.month0();
        let day = date.day0();

        let time = naive_date_time.time();
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();

        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

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
            Timestamp::from_naive_date_time(
                NaiveDateTime::parse_from_str(&raw_datetime, "%d.%m.%y, %H:%M:%S")
                    .map_err(|err| {
                        eprintln!("input: {:?}", raw_datetime);
                        err
                    })
                    // TODO return proper error
                    .unwrap(),
            ),
        ))
    }

    /// Get the year, month and day of the `Timestamp`
    ///
    /// Month and day starts at 0.
    pub fn ymd(&self) -> (i32, u32, u32) {
        (self.year, self.month, self.day)
    }

    /// Get the hours, minutes and seconds of the `Timestamp`
    pub fn hms(&self) -> (u32, u32, u32) {
        (self.hour, self.minute, self.second)
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
            Timestamp::from_naive_date_time(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(1, 2, 3).unwrap()
            ))
        )
    }

    #[test]
    fn cant_parse() {
        let out = Timestamp::parse(b"[08:58, 12.12.2018]");
        assert!(out.is_err());
    }
}
