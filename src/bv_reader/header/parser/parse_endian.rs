use regex::Regex;
use std::sync::OnceLock;


static DATAFORMAT_REGEX: OnceLock<Regex> = OnceLock::new();

/// Returns false if endian was not specified -> use little endian if false
pub fn parse_endian(textcontent: &str) -> bool {

    let re = DATAFORMAT_REGEX.get_or_init(|| {
        Regex::new(r"UseBigEndianOrder=(\w*)").unwrap() 
      });
    
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(false, |m| {
            match m.as_str() {
                "YES" => {true},
                "NO" => {false},
                _ => {false},
            }
        })
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_endian() {
        let input = "[Common Infos]
        Codepage=UTF-8
        DataFile=01_data.eeg
        MarkerFile=01_marker.vmrk
        DataFormat=BINARY
        ; Data orientation: MULTIPLEXED=ch1,pt1, ch2,pt1 ...
        DataOrientation=MULTIPLEXED
        NumberOfChannels=71
        ; Sampling interval in microseconds
        SamplingInterval=2000
        UseBigEndianOrder=YES";

        let output = parse_endian(input);
        let expected = true;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_endian_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFile=01_data.eeg";
        let output = parse_endian(input);
        let expected = false;
        assert_eq!(output, expected);
    }

}