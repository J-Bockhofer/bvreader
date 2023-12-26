use regex::Regex;
use std::sync::OnceLock;

static MARKER_VERSION_REGEX: OnceLock<Regex> = OnceLock::new();

/// Returns an empty String if no version was found
pub fn parse_marker_version(textcontent: &str) -> Option<String> {

    let re = MARKER_VERSION_REGEX.get_or_init(|| {
        Regex::new(r"Brain Vision Data Exchange Marker File, Version (\d{1,}\.\d{1,})").unwrap() 
      });
    
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(Option::None, |m| Some(m.as_str().to_string()))
    } else {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_marker_version() {
        let input = "Brain Vision Data Exchange Marker File, Version 1.0";
        let output = parse_marker_version(input).unwrap();
        let expected = "1.0".to_string();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_marker_version_empty() {
        let input = "Brain Vision Data Exchange Marker File Version";
        let output = parse_marker_version(input);
        let expected = Option::None;
        assert_eq!(output, expected);
    }

}