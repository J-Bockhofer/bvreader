use regex::Regex;
use std::sync::OnceLock;

static DATAFILE_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns the datafile path or Option::None
pub fn parse_datafilepath(textcontent: &str) -> Option<String> {
    let re = DATAFILE_REGEX.get_or_init(|| {
        Regex::new(r"DataFile=([\w\.-]*)").unwrap() 
      }); 
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(Option::None, |m|  Some(m.as_str().to_string()) )
    } else {
        Option::None
    }
}

static MARKERFILE_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns the markerfile path or Option::None
pub fn parse_markerfilepath(textcontent: &str) -> Option<String> {
    let re = MARKERFILE_REGEX.get_or_init(|| {
        Regex::new(r"MarkerFile=([\w\.-]*)").unwrap() 
      }); 
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(Option::None, |m|  Some(m.as_str().to_string()) )
    } else {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_datafilepath() {
        let input = "[Common Infos]
        Codepage=UTF-8
        DataFile=01_data.eeg";
        let output = parse_datafilepath(input).unwrap();
        let expected = String::from("01_data.eeg");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_markerfilepath() {
        let input = "[Common Infos]
        Codepage=UTF-8
        DataFile=01_data.eeg
        MarkerFile=01_marker.vmrk
        DataFormat=BINARY
        ; Data orientation: MULTIPLEXED=ch1,pt1, ch2,pt1 ...
        DataOrientation=MULTIPLEXED
        NumberOfChannels=71
        ; Sampling interval in microseconds
        SamplingInterval=2000";
        let output = parse_markerfilepath(input).unwrap();
        let expected = String::from("01_marker.vmrk");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_datafilepath_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFil";
        let output = parse_datafilepath(input);
        let expected = Option::None;
        assert_eq!(output, expected);
    }

}