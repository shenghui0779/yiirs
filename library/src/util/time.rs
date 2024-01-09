use chrono::prelude::*;

pub enum Layout<'a> {
    Date(Option<&'a str>),
    Time(Option<&'a str>),
    DateTime(Option<&'a str>),
}

// 默认格式：%Y-%m-%d %H:%M:%S
impl<'a> Layout<'a> {
    pub fn to_str(self) -> &'a str {
        match self {
            Layout::Date(v) => v.unwrap_or("%Y-%m-%d"),
            Layout::Time(v) => v.unwrap_or("%H:%M:%S"),
            Layout::DateTime(v) => v.unwrap_or("%Y-%m-%d %H:%M:%S"),
        }
    }
}

pub struct Format<'a>(pub Layout<'a>);

impl<'a> Format<'a> {
    // Unix时间戳格式化
    pub fn to_string(self, timestamp: i64) -> String {
        let Format(layout) = self;
        let timezone = FixedOffset::east_opt(8 * 3600).unwrap();

        if timestamp < 0 {
            return Utc::now()
                .with_timezone(&timezone)
                .format(layout.to_str())
                .to_string();
        }

        match DateTime::<Utc>::from_timestamp(timestamp, 0) {
            None => String::from(""),
            Some(v) => v
                .with_timezone(&timezone)
                .format(layout.to_str())
                .to_string(),
        }
    }

    // 日期转Unix时间戳
    pub fn to_timestamp(self, datetime: &str) -> i64 {
        let Format(layout) = self;

        if datetime.len() == 0 {
            return 0;
        }

        let timezone = FixedOffset::east_opt(8 * 3600).unwrap();

        match layout {
            Layout::Date(v) => match NaiveDate::parse_from_str(datetime, v.unwrap_or("%Y-%m-%d")) {
                Err(_) => 0,
                Ok(v) => v
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(timezone)
                    .unwrap()
                    .timestamp(),
            },
            Layout::DateTime(v) => {
                match NaiveDateTime::parse_from_str(datetime, v.unwrap_or("%Y-%m-%d %H:%M:%S")) {
                    Err(_) => 0,
                    Ok(v) => v.and_local_timezone(timezone).unwrap().timestamp(),
                }
            }
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::time::Layout;

    use super::Format;

    #[test]
    fn time_to_str() {
        // date
        assert_eq!(
            Format(Layout::Date(None)).to_string(1689140713),
            "2023-07-12"
        );
        assert_eq!(
            Format(Layout::Date(Some("%Y/%m/%d"))).to_string(1689140713),
            "2023/07/12"
        );

        // time
        assert_eq!(Format(Layout::Time(None)).to_string(1689140713), "13:45:13");
        assert_eq!(
            Format(Layout::Time(Some("%H-%M-%S"))).to_string(1689140713),
            "13-45-13"
        );

        // datetime
        assert_eq!(
            Format(Layout::DateTime(None)).to_string(1689140713),
            "2023-07-12 13:45:13"
        );
        assert_eq!(
            Format(Layout::DateTime(Some("%Y/%m/%d %H:%M:%S"))).to_string(1689140713),
            "2023/07/12 13:45:13"
        );
    }

    #[test]
    fn str_to_time() {
        // date
        assert_eq!(
            Format(Layout::Date(None)).to_timestamp("2023-07-12"),
            1689091200
        );
        assert_eq!(
            Format(Layout::Date(Some("%Y/%m/%d"))).to_timestamp("2023/07/12"),
            1689091200
        );

        // datetime
        assert_eq!(
            Format(Layout::DateTime(Some("%Y-%m-%d %H:%M"))).to_timestamp("2023-07-12 13:45"),
            1689140700
        );
        assert_eq!(
            Format(Layout::DateTime(None)).to_timestamp("2023-07-12 13:45:13"),
            1689140713
        );
        assert_eq!(
            Format(Layout::DateTime(Some("%Y/%m/%d %H:%M:%S"))).to_timestamp("2023/07/12 13:45:13"),
            1689140713
        );
    }
}
