
use std::fs::{self, File};
use std::io::Read;
//use std::fmt::Error;

use crate::bv_reader::bv_error::Error;

/// Reads file as vector of bytes
pub fn get_file_as_byte_vec(filename: &str) -> Result<Vec<u8>, Error> {

    let f = File::open(&filename);
    if f.is_err() {return Err(Error::FileOpen(filename.to_string()));}

    let metadata = fs::metadata(&filename);
    if metadata.is_err() {return Err(Error::FileMeta(filename.to_string()));}

    let mut f = f.unwrap();
    let metadata = metadata.unwrap();
    
    let mut buffer = vec![0; metadata.len() as usize];
    let res = f.read(&mut buffer);
    if res.is_err() {return Err(Error::FileRead(filename.to_string()));}

    Ok(buffer)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_as_byte_vec_empty() {
        let res = get_file_as_byte_vec("src/bv_reader/data/testfiles/test_data_empty.txt");
        assert_eq!(res.unwrap(), vec![])
    }

    #[test]
    fn test_get_file_as_byte_vec_simple() {
        let res = get_file_as_byte_vec("src/bv_reader/data/testfiles/test_data_1.txt");
        assert_eq!(res.unwrap(), vec![65]) // 65 is A
    }

    #[test]
    fn test_get_file_as_byte_vec_invalid_file() {
        let res = get_file_as_byte_vec("src/bv_reader/data/testfiles/test_rasd_empty.txt");
        assert_eq!(res, Err(Error::FileOpen("src/bv_reader/data/testfiles/test_rasd_empty.txt".to_string())))
    }

}