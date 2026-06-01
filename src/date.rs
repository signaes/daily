use chrono::{Local, NaiveDate};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Date(NaiveDate);

impl Date {
    pub fn today() -> Self {
        Date(Local::now().date_naive())
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        s.parse()
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map(Date)
            .map_err(|e| format!("Invalid date '{s}': {e}"))
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_date() {
        let date = "2026-05-31".parse::<Date>().unwrap();

        assert_eq!(date.to_string(), "2026-05-31");
    }

    #[test]
    fn parse_invalid_date() {
        assert!("2026-13-01".parse::<Date>().is_err());
    }

    #[test]
    fn parse_wrong_format() {
        assert!("05-31-2026".parse::<Date>().is_err());
    }

    #[test]
    fn today_is_valid() {
        let _ = Date::today().to_string();
    }

    #[test]
    fn equality() {
        let a = "2026-05-31".parse::<Date>().unwrap();
        let b = "2026-05-31".parse::<Date>().unwrap();
        let c = "2026-06-01".parse::<Date>().unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
