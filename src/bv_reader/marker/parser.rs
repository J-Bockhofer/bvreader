use regex::Regex;
use std::sync::OnceLock;

pub mod parse_marker_version;


#[derive(Default, Debug, PartialEq, Clone)]
pub struct MarkerData {
    pub marker_id: String,
    pub marker_type: String,
    pub marker_description: String,
    pub marker_position: usize,
    pub marker_length: usize,
    pub marker_chan: usize,
}


static MARKER_DATA_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn parse_marker_data(textcontent: &str) -> Vec<MarkerData> {
    
    let re = MARKER_DATA_REGEX.get_or_init(|| {
        Regex::new(r"(\w*)=([\w\ ]*),([\w\ ]*),(\d*),(\d*),(\d*)").unwrap() 
      });
    
    let mut results = vec![];

    // Iterate over matches and extract components
    for caps in re.captures_iter(textcontent) {
        let marker_id = caps.get(1).map(|m| m.as_str());
        if marker_id.is_some() { // serves as validation that we can safely unwrap everything in this line
            let marker_id = marker_id.unwrap().to_string();
            let marker_type = caps.get(2).map(|m| m.as_str().to_string()).unwrap();
            let marker_description = caps.get(3).map(|m| m.as_str().to_string()).unwrap();
            let marker_position = caps.get(4).map(|m| m.as_str().parse::<usize>().unwrap()).unwrap();
            let marker_length = caps.get(5).map(|m| m.as_str().parse::<usize>().unwrap()).unwrap();
            let marker_chan = caps.get(6).map(|m| m.as_str().parse::<usize>().unwrap()).unwrap();

            let chan = MarkerData { 
                marker_id, 
                marker_type, 
                marker_description, 
                marker_position, 
                marker_length, 
                marker_chan};

            results.push(chan);
            
        }    
    }
    results

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_marker_data() {
        let input = "; Fields are delimited by commas, some fields might be omitted (empty).
        ; Commas in type or description text are coded as \"\\1\".
        Mk1=New Segment,,1,1,0,20200316125805099157
        Mk2=Response,R  3,8598,1,0
        Mk3=Response,R  2,10854,1,0";
        let output = parse_marker_data(input);
        let expected = MarkerData{
            marker_id: "Mk2".to_string(),
            marker_type: "Response".to_string(),
            marker_description: "R  3".to_string(),
            marker_position: 8598,
            marker_length: 1,
            marker_chan: 0,
        };
        assert_eq!(output[1], expected);
    }

    #[test]
    fn test_parse_marker_data_empty() {
        let input = "; Fields are delimited by commas, some fields might be omitted (empty).
        ; Commas in type or description text are coded as \"\\1\".
        Mk1=
        Mk2=
        Mk3=";
        let output = parse_marker_data(input);
        let expected = vec![];
        assert_eq!(output, expected);
    }

}