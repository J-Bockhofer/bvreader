use regex::Regex;
use std::sync::OnceLock;

static RECORDER_VERSION_REGEX: OnceLock<Regex> = OnceLock::new();

/// Returns an empty String if no version was found
pub fn parse_recorder_version(textcontent: &str) -> Option<String> {

    let re = RECORDER_VERSION_REGEX.get_or_init(|| {
        Regex::new(r"BrainVision Recorder Professional   -   V. (.*)").unwrap() 
      });
    
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(Option::None, |m| Some(m.as_str().to_string()))
    } else {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_recorder_version() {
        let input = "
        [Comment]
        
        BrainVision Recorder Professional   -   V. 1.21.0201
        
        
        A m p l i f i e r  S e t u p";
        let output = parse_recorder_version(input).unwrap();
        let expected = "1.21.0201".to_string();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_recorder_version_empty() {
        let input = "
        [Comment]
        
        BrainVision Recorder Professional   -   V. 
        
        
        A m p l i f i e r  S e t u p";
        let output = parse_recorder_version(input);
        let expected = Some("".to_string());
        assert_eq!(output, expected);
    }

}