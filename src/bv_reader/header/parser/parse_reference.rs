use regex::Regex;
use std::sync::OnceLock;

use crate::bv_reader::generic_parser::parse_generic_value;

static REF_LABEL_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns the datafile path or Option::None
pub fn parse_reference_label(textcontent: &str) -> Option<String> {
    let re = REF_LABEL_REGEX.get_or_init(|| {
        Regex::new(r"Reference Channel Name = (\w*)").unwrap() 
      }); 
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(Option::None, |m|  Some(m.as_str().to_string()) )
    } else {
        Option::None
    }
}

static REF_PHYS_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns the markerfile path or Option::None
pub fn parse_reference_phys_chan(textcontent: &str) -> Option<usize> {
    let re = REF_PHYS_REGEX.get_or_init(|| {
        Regex::new(r"Reference Phys. Chn.   = (\w*)").unwrap() 
      }); 
    parse_generic_value::<usize>(textcontent, re)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ref_label() {
        let input = "70    71          71          0.298023 µV             DC              140              Off
        71    72          72          0.298023 µV             DC              140              Off
        
        Reference Channel Name = Cz
        Reference Phys. Chn.   = 24
        Good Level [kOhms]     = 10
        Bad Level [kOhms]      = 50
        MY-Button Workspace    = C:\\Vision\\Workfiles\\MYButton
        
        Amplifier ";
        let output = parse_reference_label(input).unwrap();
        let expected = String::from("Cz");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_ref_phys() {
        let input = "70    71          71          0.298023 µV             DC              140              Off
        71    72          72          0.298023 µV             DC              140              Off
        
        Reference Channel Name = Cz
        Reference Phys. Chn.   = 24
        Good Level [kOhms]     = 10
        Bad Level [kOhms]      = 50
        MY-Button Workspace    = C:\\Vision\\Workfiles\\MYButton
        
        Amplifier ";
        let output = parse_reference_phys_chan(input).unwrap();
        let expected = 24;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_ref_label_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFil";
        let output = parse_reference_label(input);
        let expected = Option::None;
        assert_eq!(output, expected);
    }

}