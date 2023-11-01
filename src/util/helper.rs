use chrono::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use std::{borrow::Cow, collections::HashMap};
use validator::ValidationError;

pub fn nonce(size: usize) -> String {
    let mut rng = rand::thread_rng();

    Alphanumeric.sample_string(&mut rng, size)
}

pub struct TimeFmt<'a>(pub &'a str);

#[allow(dead_code)]
impl<'a> TimeFmt<'a> {
    // Unix时间戳格式化 (%Y-%m-%d %H:%M:%S)
    pub fn to_string(self, timestamp: i64) -> String {
        let TimeFmt(format) = self;
        let timezone = FixedOffset::east_opt(8 * 3600).unwrap();

        if timestamp < 0 {
            return Utc::now()
                .with_timezone(&timezone)
                .format(format)
                .to_string();
        }

        match DateTime::<Utc>::from_timestamp(timestamp, 0) {
            None => String::from(""),
            Some(v) => v.with_timezone(&timezone).format(format).to_string(),
        }
    }

    // 日期转Unix时间戳 (%Y-%m-%d %H:%M:%S)
    pub fn to_time(self, datetime: &str) -> i64 {
        let TimeFmt(format) = self;

        if datetime.len() == 0 {
            return 0;
        }

        let timezone = FixedOffset::east_opt(8 * 3600).unwrap();

        match NaiveDateTime::parse_from_str(datetime, format) {
            Err(_) => 0,
            Ok(v) => v.and_local_timezone(timezone).unwrap().timestamp(),
        }
    }
}

pub fn query_page(args: &HashMap<String, String>) -> (u64, u64) {
    let mut offset: u64 = 0;
    let mut limit: u64 = 20;

    if let Some(v) = args.get("size") {
        let size = v.parse::<u64>().unwrap_or_default();

        if size > 0 {
            limit = size
        }
    }

    if limit > 100 {
        limit = 100
    }

    if let Some(v) = args.get("page") {
        let page = v.parse::<u64>().unwrap_or_default();

        if page > 0 {
            offset = (page - 1) * limit
        }
    }

    (offset, limit)
}

#[allow(dead_code)]
pub fn new_validation_err(s: String) -> ValidationError {
    return ValidationError {
        code: Cow::from(""),
        message: Some(Cow::from(s)),
        params: HashMap::new(),
    };
}

#[cfg(test)]
mod tests {
    use super::TimeFmt;

    #[test]
    fn time_to_str() {
        let datetime = TimeFmt("%Y-%m-%d %H:%M:%S").to_string(1689140713);
        assert_eq!(datetime, "2023-07-12 13:45:13")
    }

    #[test]
    fn str_to_time() {
        let timestamp = TimeFmt("%Y-%m-%d %H:%M:%S").to_time("2023-07-12 13:45:13");
        assert_eq!(timestamp, 1689140713);
    }
}
