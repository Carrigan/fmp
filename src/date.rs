use std::cmp::Ordering;

use super::yaml_rust::{Yaml};
use super::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    pub year: u8,
    pub month: u8,
    pub day: u8
}

impl ToString for Date {
    fn to_string(&self) -> String {
        format!("{:02}{:02}{:02}", self.year, self.month, self.day)
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        if other.year < self.year {
            return Ordering::Greater;
        } else if other.year > self.year {
            return Ordering::Less;
        }

        if other.month < self.month {
            return Ordering::Greater;
        } else if other.month > self.month {
            return Ordering::Less;
        }

        if other.day < self.day {
            return Ordering::Greater;
        } else if other.day > self.day {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Date {
    pub fn from_filename(filename: &str) -> Option<Date> {
        let filename_regex = Regex::new(r"^(\d{6})").ok()?;
        let captured_integer = filename_regex
            .captures(filename)?
            .get(0)?
            .as_str()
            .parse::<i64>()
            .ok()?;

        Some(Date::from_i64(captured_integer))
    }

    pub fn from_yaml(yaml: &Yaml) -> Option<Date> {
        if let Some(integer_date) = yaml["date"].as_i64() {
            return Some(Date::from_i64(integer_date));
        }

        if let Some(string_date) = yaml["date"].as_str() {
            let integer_date = string_date.parse::<i64>().ok()?;
            return Some(Date::from_i64(integer_date));
        }

        None
    }

    fn from_i64(integer_date: i64) -> Date {
        Date {
            year: (integer_date / 10000) as u8,
            month: ((integer_date / 100) % 100) as u8,
            day: (integer_date % 100) as u8
        }
    }
}

#[test]
fn test_date_from_yaml_number() {
    let content = String::from("date: 210820");
    let yml = yaml_rust::YamlLoader::load_from_str(&content).unwrap().pop().unwrap();
    let date = Date::from_yaml(&yml).unwrap();
    assert_eq!(date.year, 21);
    assert_eq!(date.month, 8);
    assert_eq!(date.day, 20);
}

#[test]
fn test_date_from_yaml_string() {
    let content = String::from("date: \"210820\"");
    let yml = yaml_rust::YamlLoader::load_from_str(&content).unwrap().pop().unwrap();
    let date = Date::from_yaml(&yml).unwrap();
    assert_eq!(date.year, 21);
    assert_eq!(date.month, 8);
    assert_eq!(date.day, 20);
}

#[test]
fn test_date_from_yaml_absent() {
    let content = String::from("something: else");
    let yml = yaml_rust::YamlLoader::load_from_str(&content).unwrap().pop().unwrap();
    let date = Date::from_yaml(&yml);
    assert!(date.is_none());
}

#[test]
fn test_date_from_yaml_bad() {
    let content = String::from("date: bad");
    let yml = yaml_rust::YamlLoader::load_from_str(&content).unwrap().pop().unwrap();
    let date = Date::from_yaml(&yml);
    assert!(date.is_none());
}

#[test]
fn test_date_from_filename_good() {
    let filename = "210820.md";
    let date = Date::from_filename(&filename).unwrap();
    assert_eq!(date.year, 21);
    assert_eq!(date.month, 8);
    assert_eq!(date.day, 20);
}

#[test]
fn test_date_from_filename_bad() {
    let filename = "120-bad-file.md";
    let date = Date::from_filename(&filename);
    assert!(date.is_none());
}
