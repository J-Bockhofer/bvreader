use crate::bv_reader::bv_error::Error;

pub fn parse_multiplexed_data(multiplexed_data: Vec<f32>, num_chan: usize) -> Result<Vec<Vec<f32>>, Error> {
    
    let mut channel_data: Vec<Vec<f32>> = Vec::with_capacity(num_chan); 

    let data_len = multiplexed_data.len();
    let chan_len = data_len / num_chan;

    for _ in 0..num_chan {
        let onechannel: Vec<f32> = Vec::with_capacity(chan_len);
        channel_data.push(onechannel);
    }

    let mut cnt = 0;    
    for bigdata in multiplexed_data {
        if cnt == num_chan {cnt = 0;} 
        channel_data[cnt].push(bigdata);
        cnt +=1;
    }
    let parsed_len = channel_data[0].len() * num_chan;
    if parsed_len != data_len {return Err(Error::BinaryOrientationError("MULTIPLEXED".to_string(), data_len, parsed_len))}
    Ok(channel_data)
}


pub fn parse_vectorized_data(vectorized_data: Vec<f32>, num_chan: usize) -> Result<Vec<Vec<f32>>, Error> {
    
    let mut channel_data: Vec<Vec<f32>> = Vec::with_capacity(num_chan); 
    
    let data_len = vectorized_data.len();
    let chan_len = data_len / num_chan;

    for _ in 0..num_chan {
        let onechannel: Vec<f32> = Vec::with_capacity(chan_len);
        channel_data.push(onechannel);
    }

    let chan_len = data_len / num_chan;

    let mut cnt = 1;  
    let mut chan_idx = 0;  
    for bigdata in vectorized_data {
        channel_data[chan_idx].push(bigdata);
        //println!("Idx {} - cnt {}", chan_idx, cnt);
        cnt +=1;
        if cnt == chan_len + 1 {cnt = 1; chan_idx += 1;} // reset if the next iteration is outside the values for this channel
    }
    let parsed_len = channel_data[0].len() * num_chan;
    //assert_eq!(channel_data[0].len() * num_chan, data_len);
    if parsed_len != data_len {return Err(Error::BinaryOrientationError("VECTORIZED".to_string(), data_len, parsed_len))}
    Ok(channel_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_multiplexed() {
        let res = parse_multiplexed_data(vec![
            110., 
            80.,
            20., 
            30., 
            110., 
            80., 
            20., 
            30.], 2).unwrap();
        assert_eq!(res, vec![vec![
            110., 20., 110., 20.,
        ],vec![
            80., 30., 80., 30.,
        ]])
    }

    #[test]
    fn test_parse_multiplexed_empty() {
        let res = parse_multiplexed_data(vec![], 2).unwrap();
        let expected: Vec<Vec<f32>>  = vec![vec![],vec![]];
        assert_eq!(res, expected) 

    }


    #[test]
    fn test_parse_vectorized() {
        let res = parse_vectorized_data(vec![
            110., 
            80.,
            20., 
            30., 
            110., 
            80., 
            20., 
            30.], 2).unwrap();
        assert_eq!(res, vec![vec![
            110., 80., 20., 30.,
        ],vec![
            110., 80., 20., 30.,
        ]])
    }

    #[test]
    fn test_parse_vectorized_empty() {
        let res: Vec<Vec<f32>> = parse_vectorized_data(vec![], 2).unwrap();
        let expected: Vec<Vec<f32>>  = vec![vec![],vec![]];
        assert_eq!(res, expected) 
    }
}