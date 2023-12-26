use regex::Regex;
use std::sync::OnceLock;

use crate::bv_reader::generic_parser::parse_generic_value;

static GOOD_LEVEL_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns an empty String if no version was found
pub fn parse_good_level(textcontent: &str) -> Option<usize> {

    let re = GOOD_LEVEL_REGEX.get_or_init(|| {
        Regex::new(r"Good Level \[kOhms\]     = (\d*)").unwrap() 
      });
    
    parse_generic_value::<usize>(textcontent, re)
}

static BAD_LEVEL_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns an empty String if no version was found
pub fn parse_bad_level(textcontent: &str) -> Option<usize> {

    let re = BAD_LEVEL_REGEX.get_or_init(|| {
        Regex::new(r"Bad Level \[kOhms\]      = (\d*)").unwrap() 
      });
    
    parse_generic_value::<usize>(textcontent, re)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_good_level() {
        let input = "
        Reference Channel Name = Cz
        Reference Phys. Chn.   = 24
        Good Level [kOhms]     = 10
        Bad Level [kOhms]      = 50
        MY-Button Workspace    = C:\\Vision\\Workfiles\\MYButton";
        let output = parse_good_level(input).unwrap();
        let expected = 10;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_good_level_empty() {
        let input = "
        Reference Channel Name = Cz
        Reference Phys. Chn.   = 24
        Good Level [kOhms]     = 
        Bad Level [kOhms]      = 
        MY-Button Workspace    = C:\\Vision\\Workfiles\\MYButton";
        let output = parse_good_level(input);
        let expected = Option::None;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_bad_level() {
        let input = "
        Reference Channel Name = Cz
        Reference Phys. Chn.   = 24
        Good Level [kOhms]     = 10
        Bad Level [kOhms]      = 50
        MY-Button Workspace    = C:\\Vision\\Workfiles\\MYButton";
        let output = parse_bad_level(input).unwrap();
        let expected = 50;
        assert_eq!(output, expected);
    }



}