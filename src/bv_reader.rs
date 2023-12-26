pub mod data;

pub mod header;

pub mod marker;

pub mod bv_error;

pub mod generic_parser;

use header::BVheader;
use marker::BVMarker;
use data::BVData;
use bv_error::Error;

/// Main struct of the file reader
/// 
/// Combines Header, Marker and Data
/// 
///  
pub struct BVFile {
    pub bv_header: BVheader,
    pub bv_marker: BVMarker,
    pub bv_data: BVData,
}


impl BVFile {
    /// Main function for the file reader
    /// 
    /// Takes the header filename and returns the fully parsed struct or an error if anything went wrong
    pub fn from_header(headerfile: &str) -> Result<Self, Error> {

        let root_path: Vec<&str> = headerfile.split("/").collect();
        let root_path = &root_path[0..root_path.len()-1];


        let bv_header = BVheader::from_file(headerfile)?;
        let marker_path = format!("{}/{}",root_path.join("/"),&bv_header.marker_file);
        let bv_marker = BVMarker::from_file(&marker_path)?;

        let data_path = format!("{}/{}",root_path.join("/"),&bv_header.data_file);
        let bv_data = BVData::from_file(&data_path, bv_header.num_channels, bv_header.binary_format ,bv_header.data_orientation)?;

        Ok(BVFile{
            bv_header,
            bv_marker,
            bv_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bv_file() {
        let input = "src/bv_reader/data/testfiles/01_header.vhdr";
        let output = BVFile::from_header(input).unwrap();
        let expected: usize = 71;
        //println!("{:?}", output);
        assert_eq!(output.bv_header.num_channels, expected);
    }

    #[test]
    fn test_parse_bv_file_empty() {
        let input = "";
        let output = BVFile::from_header(input);
        assert_eq!(output.is_err(), true);
    }

}
