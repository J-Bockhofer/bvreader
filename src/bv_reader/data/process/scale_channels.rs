
use crate::bv_reader::header::parser::parse_chan_info::ChannelInfo;

//use std::fmt::Error;

use crate::bv_reader::bv_error::Error;

pub fn scale_channels(data: &mut Vec<Vec<f64>>, info: &Vec<ChannelInfo>) -> Result<(), Error> {
    if data.len() != info.len() {return Err(Error::ChannelDataMismatch(data.len(), info.len()))}; // not the same amount of channels in data and info
    let chan_len = data[0].len();

    for i in 0..data.len() {
        if data[i].len() != chan_len {return Err(Error::ChannelDataMalformed(chan_len, data[i].len()))} // data malformed, unequal sample length per channel
        // iterate through channels
        let resolution = info[i].resolution;
        if resolution.is_none() {continue} // dont need to scale if there is no scale available
        let resolution = f64::from(resolution.unwrap());
        for j in 0..chan_len {
            data[i][j] *= resolution; 
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_channels() {
        let mut in_data: Vec<Vec<f64>> = vec![vec![
            110., 20., 110., 20.,
        ],vec![
            80., 30., 80., 30.,
        ]];
        let mut in_info = ChannelInfo::default();
        in_info.resolution = Some(0.5);


        scale_channels(&mut in_data, &vec![in_info.clone(), in_info]).unwrap();

        let expected: Vec<Vec<f64>> = vec![vec![
            55., 10., 55., 10.,
        ],vec![
            40., 15., 40., 15.,
        ]];

        assert_eq!(in_data, expected)

    }
}