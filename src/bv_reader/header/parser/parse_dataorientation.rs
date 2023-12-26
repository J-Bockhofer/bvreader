use regex::Regex;
use std::sync::OnceLock;



#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataOrientation{
    #[default]
    MULTIPLEXED,
    VECTORIZED,
    Unknown,
}

static DATAORIENTATION_REGEX: OnceLock<Regex> = OnceLock::new();

/// Returns DataOrientation::Unknown if no valid orientation was found
pub fn parse_data_orientation(textcontent: &str) -> DataOrientation {

    let re = DATAORIENTATION_REGEX.get_or_init(|| {
        Regex::new(r"DataOrientation=(\w*)").unwrap() 
      });
    
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(DataOrientation::Unknown, |m| {
            match m.as_str() {
                "MULTIPLEXED" => DataOrientation::MULTIPLEXED,
                "VECTORIZED" => DataOrientation::VECTORIZED,
                _ => DataOrientation::Unknown,
            }
        })
    } else {
        DataOrientation::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dataorientation() {
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
        let output = parse_data_orientation(input);
        let expected = DataOrientation::MULTIPLEXED;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_orientation_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFile=01_data.eeg";
        let output = parse_data_orientation(input);
        let expected = DataOrientation::Unknown;
        assert_eq!(output, expected);
    }

}