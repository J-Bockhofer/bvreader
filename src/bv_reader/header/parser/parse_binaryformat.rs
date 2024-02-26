use regex::Regex;
use std::sync::OnceLock;

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryFormat{
    #[default]
    IEEE_FLOAT_32,
    INT_16,
    UINT_16,
    Unknown,
}

static BINFORMAT_REGEX: OnceLock<Regex> = OnceLock::new();

/// Returns BinaryFormat::Unknown if no valid format was found
pub fn parse_binaryformat(textcontent: &str) -> BinaryFormat {

    let re = BINFORMAT_REGEX.get_or_init(|| {
        Regex::new(r"BinaryFormat=(\w*)").unwrap() 
      });
    
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(BinaryFormat::Unknown, |m| {
            match m.as_str() { 
                "IEEE_FLOAT_32" => BinaryFormat::IEEE_FLOAT_32,
                "INT_16" => BinaryFormat::INT_16,
                "UINT_16" => BinaryFormat::UINT_16,
                _ => BinaryFormat::Unknown
            }
        })
    } else {
        BinaryFormat::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_binaryformat() {
        let input = "Brain Vision Data Exchange Header File Version 1.0
        ; Data created by the Vision Recorder
        
        [Common Infos]
        Codepage=UTF-8
        DataFile=01_data.eeg
        MarkerFile=01_marker.vmrk
        DataFormat=BINARY
        ; Data orientation: MULTIPLEXED=ch1,pt1, ch2,pt1 ...
        DataOrientation=MULTIPLEXED
        NumberOfChannels=71
        ; Sampling interval in microseconds
        SamplingInterval=2000
        
        [Binary Infos]
        BinaryFormat=IEEE_FLOAT_32
        
        [Channel Infos]
        ; Each entry: Ch<Channel number>=<Name>,<Reference channel name>,
        ; <Resolution in \"Unit\">,<Unit>, Future extensions..
        ; Fields are delimited by commas, some fields might be omitted (empty).
        ; Commas in channel names are coded as \"\\1\".
        Ch1=Fp1,,0.0488281,µV
        Ch2=Fz,,0.0488281,µV
        Ch3=F3,,0.0488281,µV
        ";
        let output = parse_binaryformat(input);
        let expected = BinaryFormat::IEEE_FLOAT_32;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_binaryformat_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFile=01_data.eeg";
        let output = parse_binaryformat(input);
        let expected = BinaryFormat::Unknown;
        assert_eq!(output, expected);
    }

}