//use std::env;
use std::fs;
//use std::fmt::Error;
use crate::bv_reader::bv_error::Error;


pub mod parser;

// structs and enums
use parser::{
    parse_header_encoding::HeaderEncoding, 
    parse_dataformat::DataFormat, 
    parse_dataorientation::DataOrientation,
    parse_binaryformat::BinaryFormat,
    parse_chan_info::ChannelInfo,
    //parse_chan_info_full::ChannelInfoFull,
};

// functions
use parser::{
    parse_header_version::parse_header_version,
    parse_header_encoding::parse_header_encoding,
    parse_filepaths::{parse_datafilepath, parse_markerfilepath},
    parse_dataformat::parse_dataformat,
    parse_dataorientation::parse_data_orientation,
    parse_numchan::parse_numchans,
    parse_sinterval::parse_sampling_interval,
    parse_binaryformat::parse_binaryformat,
    parse_chan_info::parse_chan_info,
    parse_recorder_version::parse_recorder_version,
    parse_amp_setup::{parse_amp_numchan, parse_amp_samplinginterval, parse_amp_samplingrate},
    //parse_chan_info_full::parse_chan_info_full,
    parse_reference::{parse_reference_label, parse_reference_phys_chan},
    parse_levels::{parse_good_level, parse_bad_level},
};

use self::parser::parse_endian::parse_endian;



#[derive(Default, Clone, Debug, PartialEq)]
pub struct BVheader {
    pub header_path: String,

    pub header_version: String,

    // Common Info
    pub header_encoding: HeaderEncoding,
    pub data_file: String,
    pub marker_file: String,
    pub data_format: DataFormat,
    pub data_orientation: DataOrientation,
    pub num_channels: usize,
    pub sampling_interval: usize,

    // Binary Format
    pub binary_format: BinaryFormat,
    pub use_big_endian: bool,

    // Channel Info
    pub channel_info: Vec<ChannelInfo>, 

    // Comment
    pub recorder_version: String,

    // Amp setup
    pub amp_channels: usize,
    pub amp_sr: usize,
    pub amp_sample_interval: usize, 

    // Channels Full
    //pub channel_info_full: Vec<ChannelInfoFull>,

    pub reference_label: String,
    pub reference_phys_chan: usize,
    //pub level_unit: String,
    pub good_level: usize,
    pub bad_level: usize, // done till here

    // not implemented, lacking example data
    //pub my_button_wd: String,
    //pub amp_info: String,

    //pub software_filters: String,

    //pub impedence_values: String,

}


impl BVheader {
    pub fn from_file(filepath: &str) -> Result<Self, Error> {
        let textcontent = fs::read_to_string(filepath);
        if textcontent.is_err() {return Err(Error::FileRead(filepath.to_string()))};
        let textcontent = textcontent.unwrap();
        let header_version = parse_header_version(&textcontent);
        if header_version.is_none() {return Err(Error::InvalidHeaderVersion)};

        Ok(BVheader{
            header_path: filepath.to_string(),
            header_version: header_version.unwrap(),
            header_encoding: parse_header_encoding(&textcontent),
            data_file: parse_datafilepath(&textcontent).unwrap_or_default(),
            marker_file: parse_markerfilepath(&textcontent).unwrap_or_default(),
            data_format: parse_dataformat(&textcontent),
            data_orientation: parse_data_orientation(&textcontent),
            num_channels: parse_numchans(&textcontent).unwrap_or_default(),
            sampling_interval: parse_sampling_interval(&textcontent).unwrap_or_default(),
            binary_format: parse_binaryformat(&textcontent),
            use_big_endian: parse_endian(&textcontent),
            channel_info: parse_chan_info(&textcontent),
            recorder_version: parse_recorder_version(&textcontent).unwrap_or_default(),
            amp_channels: parse_amp_numchan(&textcontent).unwrap_or_default(),
            amp_sr: parse_amp_samplingrate(&textcontent).unwrap_or_default(),
            amp_sample_interval: parse_amp_samplinginterval(&textcontent).unwrap_or_default(),
            //channel_info_full: parse_chan_info_full(&textcontent),
            reference_label: parse_reference_label(&textcontent).unwrap_or_default(),
            reference_phys_chan: parse_reference_phys_chan(&textcontent).unwrap_or_default(),
            good_level: parse_good_level(&textcontent).unwrap_or_default(),
            bad_level: parse_bad_level(&textcontent).unwrap_or_default(),
        })    
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header() {
        let input = "src/bv_reader/data/testfiles/01_header.vhdr";
        let output = BVheader::from_file(input).unwrap();
        let expected: usize = 71;
        //let expected = "1.0".to_string();

        //println!("{:?}", output);
        assert_eq!(output.num_channels, expected);
    }

    #[test]
    fn test_parse_header_empty() {
        let input = "";
        let output = BVheader::from_file(input);
        let expected = Err(Error::FileRead("".to_string()));
        assert_eq!(output, expected);
    }

}


