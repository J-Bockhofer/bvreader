use regex::Regex;
use std::sync::OnceLock;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderEncoding {
    #[default]
    UTF8,
    ASCII,
    Unknown,
}

static HEADER_ENCODING_REGEX: OnceLock<Regex> = OnceLock::new();

/// Returns HeaderEncoding::Unknown if no valid encoding was found
pub fn parse_header_encoding(textcontent: &str) -> HeaderEncoding {

    let re = HEADER_ENCODING_REGEX.get_or_init(|| {
        Regex::new(r"Codepage=([\w-]*)").unwrap() 
      });
    
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(HeaderEncoding::Unknown, |m| {
            if m.as_str() == "UTF-8" {
                HeaderEncoding::UTF8
            } else if m.as_str() == "ASCII" {
                HeaderEncoding::ASCII
            } else {
                HeaderEncoding::Unknown
            }
        })
    } else {
        HeaderEncoding::Unknown
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header_encoding() {
        let input = "Brain Vision Data Exchange Header File Version 1.0
        ; Data created by the Vision Recorder
        
        [Common Infos]
        Codepage=UTF-8
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
        ; <Resolution in Unit>,<Unit>, Future extensions..
        ; Fields are delimited by commas, some fields might be omitted (empty).
        ; Commas in channel names are coded as \\1.
        Ch1=Fp1,,0.0488281,µV
        Ch2=Fz,,0.0488281,µV
        Ch3=F3,,0.0488281,µV
        Ch4=F7,,0.0488281,µV
        Ch5=FT9,,0.0488281,µV
        Ch6=FC5,,0.0488281,µV
        Ch7=FC1,,0.0488281,µV
        Ch8=C3,,0.0488281,µV
        Ch9=T7,,0.0488281,µV";
        let output = parse_header_encoding(input);
        let expected = HeaderEncoding::UTF8;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_header_encoding_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFile=sadasdffs";
        let output = parse_header_encoding(input);
        let expected = HeaderEncoding::Unknown;
        assert_eq!(output, expected);
    }

}