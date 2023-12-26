use regex::Regex;
use std::sync::OnceLock;

use crate::bv_reader::generic_parser::parse_generic_value;

static NUMCHAN_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns the number of channels or Option::None
pub fn parse_numchans(textcontent: &str) -> Option<usize> {
    let re = NUMCHAN_REGEX.get_or_init(|| {
        Regex::new(r"NumberOfChannels=(\d*)").unwrap() 
      }); 
    parse_generic_value::<usize>(textcontent, re)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numchans() {
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
        let output = parse_numchans(input).unwrap();
        let expected = 71;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_numchans_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFil";
        let output = parse_numchans(input);
        let expected = Option::None;
        assert_eq!(output, expected);
    }

}