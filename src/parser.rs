use regex::Regex;

pub fn parse_artists(str: &str) -> Result<Vec<&str>, anyhow::Error> {
    // TODO: ditch regex, no need to depend on a crate only for this
    // TODO: should be reused instead of creating a new one
    // TODO: support comma separated artists
    let regex = Regex::new(r"(?i)\s(feat\.?|ft\.?|featuring|with|&|and|x)\s")?;

    Ok(regex.split(str).collect())
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_artists;

    #[test]
    fn test_parse_artists() {
        assert_eq!(parse_artists("Artist One").unwrap(), vec!["Artist One"]);
        assert_eq!(
            parse_artists("Artist One feat. Artist Two").unwrap(),
            vec!["Artist One", "Artist Two"]
        );
        assert_eq!(
            parse_artists("Artist One & Artist Three").unwrap(),
            vec!["Artist One", "Artist Three"]
        );
    }
}
