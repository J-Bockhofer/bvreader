use regex::Regex;
use std::sync::OnceLock;


#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataFormat {
    #[default]
    BINARY,
    Unknown,
}

static DATAFORMAT_REGEX: OnceLock<Regex> = OnceLock::new();

/// Returns DataFormat::Unknown if no valid format was found
pub fn parse_dataformat(textcontent: &str) -> DataFormat {

    let re = DATAFORMAT_REGEX.get_or_init(|| {
        Regex::new(r"DataFormat=(\w*)").unwrap() 
      });
    
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(DataFormat::Unknown, |m| {
            if m.as_str() == "BINARY" {
                DataFormat::BINARY
            } else {

                DataFormat::Unknown
            }
        })
    } else {
        DataFormat::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dataformat() {
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
        let output = parse_dataformat(input);
        let expected = DataFormat::BINARY;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_dataformat_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFile=01_data.eeg";
        let output = parse_dataformat(input);
        let expected = DataFormat::Unknown;
        assert_eq!(output, expected);
    }

}