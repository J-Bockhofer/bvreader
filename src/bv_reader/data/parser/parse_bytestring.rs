



pub fn parse_bytestring_to_f32(bytestring: Vec<u8>, use_big_endian: bool) -> Vec<f64> {

    let bytestring_len = bytestring.len();

    if bytestring_len == 0 {return vec![]}
    const CHUNK_SIZE: usize = 4;
    let mut result: Vec<f64> = Vec::with_capacity(bytestring_len / CHUNK_SIZE);

    let mut byte_idx = 0;
    while byte_idx + CHUNK_SIZE - 1 <= bytestring_len - 1 {

        let new_head = byte_idx + CHUNK_SIZE;
        let chunk: &[u8] = &bytestring[byte_idx..new_head];
        let num = if use_big_endian {f32::from_be_bytes(format_4byte_arr(chunk))} else {f32::from_le_bytes(format_4byte_arr(chunk))}; 
        result.push(f64::from(num));

        byte_idx = new_head;
    }
    assert_eq!(byte_idx, bytestring_len);
    result
}

pub fn parse_bytestring_to_i32(bytestring: Vec<u8>, use_big_endian: bool) -> Vec<f64> {

    let bytestring_len = bytestring.len();
    if bytestring_len == 0 {return vec![]}
    const CHUNK_SIZE: usize = 4;
    let mut result: Vec<f64> = Vec::with_capacity(bytestring_len / CHUNK_SIZE);
    let mut byte_idx = 0;

    while byte_idx + CHUNK_SIZE - 1 <= bytestring_len - 1 {
        let new_head = byte_idx + CHUNK_SIZE;
        let chunk: &[u8] = &bytestring[byte_idx..new_head];

        let num = if use_big_endian {i32::from_be_bytes(format_4byte_arr(chunk))} else {i32::from_le_bytes(format_4byte_arr(chunk))};
        result.push(f64::from(num));

        byte_idx = new_head;
    }
    assert_eq!(byte_idx, bytestring_len);
    result
}

pub fn parse_bytestring_to_i16(bytestring: Vec<u8>, use_big_endian: bool) -> Vec<f64> {

    let bytestring_len = bytestring.len();
    if bytestring_len == 0 {return vec![]}
    const CHUNK_SIZE: usize = 2;
    let mut result: Vec<f64> = vec![];
    let mut byte_idx = 0;

    while byte_idx + CHUNK_SIZE - 1 <= bytestring_len - 1 {

        let new_head = byte_idx + CHUNK_SIZE;
        let chunk: &[u8] = &bytestring[byte_idx..new_head];
        let num = if use_big_endian {i16::from_be_bytes(format_2byte_arr(chunk))} else {i16::from_le_bytes(format_2byte_arr(chunk))};
        result.push(f64::from(num));

        byte_idx = new_head;
    }
    assert_eq!(byte_idx, bytestring_len);
    result
}

pub fn format_4byte_arr(chunk: &[u8]) -> [u8; 4] {
    chunk.try_into().expect("Not of matching size")
}

pub fn format_2byte_arr(chunk: &[u8]) -> [u8; 2] {
    chunk.try_into().expect("Not of matching size")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bytestring_f32() {
        let res = parse_bytestring_to_f32(vec![110, 80, 20, 30, 110, 80, 20, 30], false);
        let expected = vec![7.851687231645703e-21, 7.851687231645703e-21];
        assert_eq!(res, expected)
    }

    #[test]
    fn test_parse_bytestring_i32() {
        let res= parse_bytestring_to_i32(vec![110, 80, 20, 30, 110, 80, 20, 30], false);
        let expected = vec![504647790., 504647790.];
        assert_eq!(res, expected)
    }

    #[test]
    fn test_parse_bytestring_i16() {
        let res = parse_bytestring_to_i16(vec![110, 80, 20, 30, 110, 80, 20, 30], false);
        let expected = vec![20590., 7700., 20590., 7700.];
        assert_eq!(res, expected)
    }

    #[test]
    fn test_parse_bytestring_f32_empty() {
        let res: Vec<f64> = parse_bytestring_to_f32(vec![], false);
         let expected: Vec<f64> = vec![];
        assert_eq!(res, expected) 
    }

    #[test]
    fn test_format_4byte_chunk() {
        let res = format_4byte_arr(&[12,12,12,12]);
        assert_eq!(res, [12; 4])
    }

    #[test]
    fn test_format_2byte_chunk() {
        let res = format_2byte_arr(&[12,12]);
        assert_eq!(res, [12; 2])
    }

}