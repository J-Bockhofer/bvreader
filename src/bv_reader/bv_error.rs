use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {

    // File operations
    #[error("Could not open file @ {0}")]
    FileOpen(String),
    #[error("Could not read file metadata @ {0}")]
    FileMeta(String),
    #[error("Could not read file @ {0}")]
    FileRead(String),

    // Header + Marker
    #[error("Could not parse header version")]
    InvalidHeaderVersion,

    // Data 
    #[error("Invalid binary format")]
    InvalidBinaryFormat,
    #[error("Empty binary file")]
    EmptyBinary,
    #[error("Invalid data orientation")]
    InvalidDataOrientation,

    // Binary parser
    #[error("Invalid binary dimensions for binary with format {0}. Parser stopped at byte {1} while binary contains {2} bytes")]
    BinaryParserError(String, usize, usize),
    #[error("Invalid binary data orientation for orientation {0}. Parser expected {1} values and got {2} values")]
    BinaryOrientationError(String, usize, usize),


    // Channel scaling
    #[error("Mismatching channel data dimensions. Data has length: {0} while resolution info has length: {1}")]
    ChannelDataMismatch(usize, usize),
    #[error("Malformed channel data. Expected sample length per channel: {0}. Found: {1}")]
    ChannelDataMalformed(usize, usize),

    // Validation
    #[error("{0}")]
    ValidationError(String),
    


}
