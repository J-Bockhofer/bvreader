pub mod parser;

#[allow(unused_imports)]
use parser::{
    read_datfile::get_file_as_byte_vec,
    parse_bytestring::{parse_bytestring_to_f32, parse_bytestring_to_i16, parse_bytestring_to_i32},
    parse_orientation::{parse_multiplexed_data, parse_vectorized_data}
};

pub mod process;

use process::scale_channels::scale_channels;

use crate::bv_reader::header::parser::{
    parse_dataorientation::DataOrientation,
    parse_binaryformat::BinaryFormat,
    parse_chan_info::ChannelInfo};

use crate::bv_reader::bv_error::Error;



/// Stores the data as vector of channels, each containing the vector of samples
#[derive(Debug, PartialEq)]
pub struct BVData {
    pub data_path: String,
    pub data: Vec<Vec<f32>>, // 2D-Vector of channels
    pub num_chan: usize,
}

impl BVData {
    pub fn from_file(datapath: &str, num_channels: usize, binary_format: BinaryFormat, orientation: DataOrientation, use_big_endian: bool) -> Result<Self, Error> {
        let bytestring = get_file_as_byte_vec(datapath)?;

        let raw_data = match binary_format {
            BinaryFormat::IEEE_FLOAT_32 => {parse_bytestring_to_f32(bytestring, use_big_endian)},
            BinaryFormat::INT_32 => {parse_bytestring_to_i32(bytestring, use_big_endian)},
            BinaryFormat::INT_16 => {parse_bytestring_to_i16(bytestring, use_big_endian)},
            BinaryFormat::Unknown => {return Err(Error::InvalidBinaryFormat);}
        };
        
        if raw_data.len() == 0 {return Err(Error::EmptyBinary)}

        let parsed_data = match orientation {
            DataOrientation::MULTIPLEXED => {parse_multiplexed_data(raw_data, num_channels)},
            DataOrientation::VECTORIZED => {parse_vectorized_data(raw_data, num_channels)},
            DataOrientation::Unknown => {return Err(Error::InvalidDataOrientation);}
        };
        let num_channels = parsed_data.len();
        Ok(BVData{
            data_path: datapath.to_string(),
            data: parsed_data,
            num_chan: num_channels,
        })
    }

    pub fn scale_channels(&mut self, channel_info: &Vec<ChannelInfo>) -> Result<(), Error> {
        scale_channels(&mut self.data, channel_info)?;
        Ok(())
    }
}