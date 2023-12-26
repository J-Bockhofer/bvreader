use regex::Regex;
use std::sync::OnceLock;

use crate::bv_reader::generic_parser::parse_generic_value;

static SINTER_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns the sampling interval or Option::None
pub fn parse_sampling_interval(textcontent: &str) -> Option<usize> {
    let re = SINTER_REGEX.get_or_init(|| {
        Regex::new(r"SamplingInterval=(\d*)").unwrap() 
      }); 
    parse_generic_value::<usize>(textcontent, re)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sampling_interval() {
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
        let output = parse_sampling_interval(input).unwrap();
        let expected = 2000;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_sampling_interval_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFil";
        let output = parse_sampling_interval(input);
        let expected = Option::None;
        assert_eq!(output, expected);
    }

}