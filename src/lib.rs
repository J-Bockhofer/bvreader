//! # bvreader
//! 
//! This crate provides a basic reader for BrainVision Recorder Files `(.vhdr/.vmrk/.eeg)`
//!
//! `ONLY FOR HEADER VERSION 1.0`
//!
//! Does not include full list of options but is extendable.
//!
//! In order to keep it simple all types (FLOAT32, INT32, INT16) convert to f32.
//!
//! Only implements processing for `Timedomain Data`.
//!
//! See the bv_reader module for usage instructions.
//!
//! 

/// ## Usage
/// ```
/// 
/// use crate::bvreader::bv_reader::BVFile;
/// 
/// let headerfile = "src/bv_reader/data/testfiles/01_header.vhdr";
/// let metafile = BVFile::from_header(headerfile).unwrap();
/// 
/// // Optionally validate the BVFile struct
/// let metafile = metafile.validate().unwrap();
/// 
/// // metafile.bv_header       contains the struct with header information
/// // metafile.bv_data         contains the data, filepath and number of channels
/// // metafile.bv_marker       contains the marker events
/// 
/// ```
pub mod bv_reader;


