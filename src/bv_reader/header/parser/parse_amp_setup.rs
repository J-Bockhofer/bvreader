use regex::Regex;
use std::sync::OnceLock;

use crate::bv_reader::generic_parser::parse_generic_value;

static AMP_NUMCHAN_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns None if no version was found
pub fn parse_amp_numchan(textcontent: &str) -> Option<usize> {

    let re = AMP_NUMCHAN_REGEX.get_or_init(|| {
        Regex::new(r"Number of channels: (\d*)").unwrap() 
      });
    
    parse_generic_value::<usize>(textcontent, re)
}

static AMP_SR_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns None if no sampling rate was found
pub fn parse_amp_samplingrate(textcontent: &str) -> Option<usize> {

    let re = AMP_SR_REGEX.get_or_init(|| {
        Regex::new(r"Sampling Rate \[Hz\]: (\d*)").unwrap() 
      });
    
    parse_generic_value::<usize>(textcontent, re)
}

static AMP_SI_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns None if no sampling interval was found
pub fn parse_amp_samplinginterval(textcontent: &str) -> Option<usize> {

    let re = AMP_SI_REGEX.get_or_init(|| {
        Regex::new(r"Sampling Interval \[µS\]: (\d*)").unwrap() 
      });
    
    parse_generic_value::<usize>(textcontent, re)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_amp_numchan() {
        let input = "
        [Comment]

        BrainVision Recorder Professional   -   V. 1.21.0201
        
        
        A m p l i f i e r  S e t u p
        ============================
        Number of channels: 71
        Sampling Rate [Hz]: 500
        Sampling Interval [µS]: 2000";
        let output = parse_amp_numchan(input).unwrap();
        let expected = 71;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_amp_numchan_empty() {
        let input = "
        [Comment]

        BrainVision Recorder Professional   -   V. 1.21.0201
        
        
        A m p l i f i e r  S e t u p
        ============================
        Number of channels:
        Sampling Rate [Hz]:
        Sampling Interval [µS]:";
        let output = parse_amp_numchan(input);
        let expected = Option::None;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_amp_samplingrate() {
        let input = "
        [Comment]

        BrainVision Recorder Professional   -   V. 1.21.0201
        
        
        A m p l i f i e r  S e t u p
        ============================
        Number of channels: 71
        Sampling Rate [Hz]: 500
        Sampling Interval [µS]: 2000";
        let output = parse_amp_samplingrate(input).unwrap();
        let expected = 500;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_amp_samplingrate_empty() {
        let input = "
        [Comment]

        BrainVision Recorder Professional   -   V. 1.21.0201
        
        
        A m p l i f i e r  S e t u p
        ============================
        Number of channels:
        Sampling Rate [Hz]:
        Sampling Interval [µS]:";
        let output = parse_amp_samplingrate(input);
        let expected = Option::None;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_amp_samplinginterval() {
        let input = "
        [Comment]

        BrainVision Recorder Professional   -   V. 1.21.0201
        
        
        A m p l i f i e r  S e t u p
        ============================
        Number of channels: 71
        Sampling Rate [Hz]: 500
        Sampling Interval [µS]: 2000";
        let output = parse_amp_samplinginterval(input).unwrap();
        let expected = 2000;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_amp_samplinginterval_empty() {
        let input = "
        [Comment]

        BrainVision Recorder Professional   -   V. 1.21.0201
        
        
        A m p l i f i e r  S e t u p
        ============================
        Number of channels:
        Sampling Rate [Hz]:
        Sampling Interval [µS]:";
        let output = parse_amp_samplinginterval(input);
        let expected = Option::None;
        assert_eq!(output, expected);
    }

}