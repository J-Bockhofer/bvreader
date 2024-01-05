//! 
//! This module contains functions for validating the BVFile struct
//! 

use super::{BVFile, Error};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IsValid {
    True,
    False(String),
}

/// Parses the invalid enum variant of IsValid to Error::ValidationError
/// 
/// Will always return Error, even if the value was IsValid::True
pub fn parse_invalid_to_error(value: &IsValid) -> Error {
    if let IsValid::False(ref msg) = value {
        Error::ValidationError(msg.to_string())
    } else {
        Error::ValidationError("".to_string())
    }
}

/// Validates that the number of channels is equal across all entries
/// 
/// 
pub fn validate_num_chan(bvfile: &BVFile) -> IsValid {
    let base_err = "Channel mismatch";
    let num_chan = bvfile.bv_header.num_channels;
    if num_chan != bvfile.bv_header.amp_channels {
        return IsValid::False(
            format!("{base_err}: channels in header {}, channels in AMP {}", num_chan, bvfile.bv_header.amp_channels)
        )
    };

    if num_chan != bvfile.bv_header.channel_info.len() {
        return IsValid::False(
            format!("{base_err}: channels in header {}, channels in channel info {}", num_chan, bvfile.bv_header.channel_info.len())
        )
    };

    if num_chan != bvfile.bv_data.data.len() {
        return IsValid::False(
            format!("{base_err}: channels in header {}, channels in data {}", num_chan, bvfile.bv_data.data.len())
        )
    };

    IsValid::True
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_validate_num_chan() {
        let input = "src/bv_reader/data/testfiles/01_header.vhdr";
        let bvfile = BVFile::from_header(input).unwrap();
        let output = validate_num_chan(&bvfile);

        let expected = IsValid::True;

        assert_eq!(output, expected);
    }


}

