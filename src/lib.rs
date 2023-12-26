
/// ## Usage
/// ```
/// 
/// use crate::bvreader::bv_reader::BVFile;
/// 
/// let headerfile = "src/bv_reader/data/testfiles/01_header.vhdr";
/// let metafile = BVFile::from_header(headerfile).unwrap();
/// 
/// // metafile.bv_header       contains the struct with header information
/// // metafile.bv_data         contains the data, filepath and number of channels
/// // metafile.bv_marker       contains the marker events
/// 
/// ```
pub mod bv_reader;


