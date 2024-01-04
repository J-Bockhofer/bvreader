use regex::Regex;
use std::sync::OnceLock;


#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataType {
    #[default]
    Timedomain,
    Frequencydomain,
    Unknown,
}

static DATAFORMAT_REGEX: OnceLock<Regex> = OnceLock::new();

/// Returns DataType::Timedomain if no DataType was specified
pub fn parse_datatype(textcontent: &str) -> DataType {

    let re = DATAFORMAT_REGEX.get_or_init(|| {
        Regex::new(r"DataType=(\w*)").unwrap() 
      });
    
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(DataType::Timedomain, |m| {
            match m.as_str() {
                "TIMEDOMAIN" => {DataType::Timedomain},
                "FREQUENCYDOMAIN" => {DataType::Frequencydomain},
                _ => {DataType::Unknown},
            }
        })
    } else {
        DataType::Timedomain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_datatype() {
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
        DataType=TIMEDOMAIN";
        let output = parse_datatype(input);
        let expected = DataType::Timedomain;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_datatype_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFile=01_data.eeg";
        let output = parse_datatype(input);
        let expected = DataType::Timedomain;
        assert_eq!(output, expected);
    }

}