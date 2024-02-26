

use std::fs;
//use std::fmt::Error;
use crate::bv_reader::bv_error::Error;

pub mod parser;
use parser::{
    MarkerData, 
    parse_marker_data, 
    parse_marker_version::parse_marker_version, 
    parse_timecode::{BVTime, parse_timecode}};

use crate::bv_reader::header::parser::{
    parse_header_encoding::{parse_header_encoding, HeaderEncoding},
    parse_filepaths::parse_datafilepath,
};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct BVMarker {
    pub marker_path: String,
    pub header_version: String,

    pub header_encoding: HeaderEncoding,
    pub data_file: String,

    pub marker_data: Vec<MarkerData>,
    pub start_time: Option<BVTime>,

}

impl BVMarker {
    pub fn from_file(filepath: &str) -> Result<Self, Error> {
        let textcontent = fs::read_to_string(filepath);
        if textcontent.is_err() {return Err(Error::FileRead(filepath.to_string()))}
        let textcontent = textcontent.unwrap();
        let header_version = parse_marker_version(&textcontent);
        if header_version.is_none() {return Err(Error::InvalidHeaderVersion)};

        Ok(BVMarker{
            marker_path: filepath.to_string(),
            header_version: header_version.unwrap(),
            header_encoding: parse_header_encoding(&textcontent),
            data_file: parse_datafilepath(&textcontent).unwrap(),
            marker_data: parse_marker_data(&textcontent),
            start_time: parse_timecode(&textcontent),
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_marker() {
        let input = "src/bv_reader/data/testfiles/01_marker.vmrk";
        let output = BVMarker::from_file(input).unwrap();
        let expected = MarkerData{
            marker_id: "Mk2".to_string(),
            marker_type: "Response".to_string(),
            marker_description: "R  3".to_string(),
            marker_position: 8598,
            marker_length: 1,
            marker_chan: 0,
        };
        //println!("{:?}", output);
        assert_eq!(output.marker_data[1], expected);
    }

    #[test]
    fn test_parse_marker_empty() {
        let input = "";
        let output = BVMarker::from_file(input);
        let expected = Err(Error::FileRead("".to_string()));
        assert_eq!(output, expected);        
    }

}
