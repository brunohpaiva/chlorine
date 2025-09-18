use jiff::{SpanRound, Timestamp, Unit, tz::TimeZone};

// this is a really bad way to do it I assume
pub fn relative_timestamp(
    timestamp: &Timestamp,
    _: &dyn askama::Values,
) -> ::askama::Result<String> {
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
