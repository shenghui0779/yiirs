use anyhow::Ok;

pub const DATE: &str = "[year]-[month]-[day]";
pub const TIME: &str = "[hour]:[minute]:[second]";
pub const DATETIME: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

// 获取当前时间
pub fn now(offset: time::UtcOffset) -> time::OffsetDateTime {
    time::OffsetDateTime::now_utc().to_offset(offset)
}

// 根据时间字符串生成时间对象
pub fn from_str(
    fmt: &str,
    datetime: &str,
    offset: time::UtcOffset,
) -> anyhow::Result<time::OffsetDateTime> {
    let format = time::format_description::parse(fmt)?;
    let v = time::PrimitiveDateTime::parse(datetime, &format)?.assume_offset(offset);
    Ok(v)
}

// 根据Unix时间戳生成时间对象
pub fn from_timestamp(
    timestamp: i64,
    offset: time::UtcOffset,
) -> anyhow::Result<time::OffsetDateTime> {
    if timestamp < 0 {
        return Ok(time::OffsetDateTime::now_utc().to_offset(offset));
    }
    let v = time::OffsetDateTime::from_unix_timestamp(timestamp)?.to_offset(offset);
    Ok(v)
}

// Unix时间戳格式化
pub fn to_string(fmt: &str, timestamp: i64, offset: time::UtcOffset) -> anyhow::Result<String> {
    let format = time::format_description::parse(fmt)?;
    if timestamp < 0 {
        let v = time::OffsetDateTime::now_utc()
            .to_offset(offset)
            .format(&format)?;
        return Ok(v);
    }
    let v = time::OffsetDateTime::from_unix_timestamp(timestamp)?
        .to_offset(offset)
        .format(&format)?;
    Ok(v)
}

// 日期转Unix时间戳
pub fn to_timestamp(fmt: &str, datetime: &str, offset: time::UtcOffset) -> anyhow::Result<i64> {
    if datetime.is_empty() {
        return Ok(0);
    }

    let format = time::format_description::parse(fmt)?;
    let v = time::PrimitiveDateTime::parse(datetime, &format)?
        .assume_offset(offset)
        .unix_timestamp();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use ::time::macros::offset;

    use crate::xtime;

    #[test]
    fn from_str() {
        // date
        assert_eq!(
            xtime::from_str(xtime::DATETIME, "2023-07-12 00:00:00", offset!(+8))
                .unwrap()
                .unix_timestamp(),
            1689091200
        );
        assert_eq!(
            xtime::from_str(
                "[year]/[month]/[day] [hour]:[minute]:[second]",
                "2023/07/12 00:00:00",
                offset!(+8)
            )
            .unwrap()
            .unix_timestamp(),
            1689091200
        );

        // datetime
        assert_eq!(
            xtime::from_str(xtime::DATETIME, "2023-07-12 13:45:13", offset!(+8))
                .unwrap()
                .unix_timestamp(),
            1689140713
        );
        assert_eq!(
            xtime::from_str(
                "[year]/[month]/[day] [hour]:[minute]:[second]",
                "2023/07/12 13:45:13",
                offset!(+8)
            )
            .unwrap()
            .unix_timestamp(),
            1689140713
        );
    }

    #[test]
    fn from_timestamp() {
        assert_eq!(
            xtime::from_timestamp(1689140713, offset!(+8))
                .unwrap()
                .unix_timestamp(),
            1689140713
        )
    }

    #[test]
    fn time_to_str() {
        // date
        assert_eq!(
            xtime::to_string(xtime::DATE, 1689140713, offset!(+8)).unwrap(),
            "2023-07-12"
        );
        assert_eq!(
            xtime::to_string("[year]/[month]/[day]", 1689140713, offset!(+8)).unwrap(),
            "2023/07/12"
        );

        // time
        assert_eq!(
            xtime::to_string(xtime::TIME, 1689140713, offset!(+8)).unwrap(),
            "13:45:13"
        );
        assert_eq!(
            xtime::to_string("[hour]-[minute]-[second]", 1689140713, offset!(+8)).unwrap(),
            "13-45-13"
        );

        // datetime
        assert_eq!(
            xtime::to_string(xtime::DATETIME, 1689140713, offset!(+8)).unwrap(),
            "2023-07-12 13:45:13"
        );
        assert_eq!(
            xtime::to_string(
                "[year]/[month]/[day] [hour]:[minute]:[second]",
                1689140713,
                offset!(+8)
            )
            .unwrap(),
            "2023/07/12 13:45:13"
        );
    }

    #[test]
    fn str_to_time() {
        // date
        assert_eq!(
            xtime::to_timestamp(xtime::DATETIME, "2023-07-12 00:00:00", offset!(+8)).unwrap(),
            1689091200
        );
        assert_eq!(
            xtime::to_timestamp(
                "[year]/[month]/[day] [hour]:[minute]:[second]",
                "2023/07/12 00:00:00",
                offset!(+8)
            )
            .unwrap(),
            1689091200
        );

        // datetime
        assert_eq!(
            xtime::to_timestamp(
                "[year]-[month]-[day] [hour]:[minute]",
                "2023-07-12 13:45",
                offset!(+8)
            )
            .unwrap(),
            1689140700
        );
        assert_eq!(
            xtime::to_timestamp(xtime::DATETIME, "2023-07-12 13:45:13", offset!(+8)).unwrap(),
            1689140713
        );
        assert_eq!(
            xtime::to_timestamp(
                "[year]/[month]/[day] [hour]:[minute]:[second]",
                "2023/07/12 13:45:13",
                offset!(+8)
            )
            .unwrap(),
            1689140713
        );
    }
}
