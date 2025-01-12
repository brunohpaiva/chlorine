use jiff::{tz::TimeZone, SpanRound, Timestamp, Unit};

// this is a really bad way to do it I assume
pub fn relative_timestamp(timestamp: &Timestamp) -> ::askama::Result<String> {
    let now = Timestamp::now();
    let now_zoned = now.to_zoned(TimeZone::system());

    let span = now
        .until(*timestamp)
        .map_err(|err| askama::Error::Custom(Box::new(err)))?
        .round(
            SpanRound::new()
                .relative(&now_zoned)
                .smallest(Unit::Second)
                .largest(Unit::Year),
        )
        .map_err(|err| askama::Error::Custom(Box::new(err)))?;

    Ok(format!("{span:#}"))
}
