# bvreader

[Documentation](https://docs.rs/bvreader)

This crate provides a basic reader for BrainVision Recorder Files `(.vhdr/.vmrk/.eeg)`

`ONLY FOR HEADER VERSION 1.0`

Does not include full list of options but is extendable.

In order to keep it simple all types (FLOAT32, INT32, INT16) convert to f32.

Only implements processing for `Timedomain Data`.

## Usage

```rust

use crate::bvreader::bv_reader::BVFile;

let headerfile = "src/bv_reader/data/testfiles/01_header.vhdr";
let metafile = BVFile::from_header(headerfile).unwrap();

// metafile.bv_header       contains the struct with header information
// metafile.bv_data         contains the data and some extra information
// metafile.bv_marker       contains the marker events

// metafile.bv_data.data    contains the sample values in a vec of channels, that each contain a vec of sample values as f32.

// scale data according to the resolution
let _ = metafile.bv_data.scale_channels(&metafile.bv_header.channel_info).unwrap();

```

## Todos

1. Decode Timecode in marker new segment

2. .ini based parser instead of regex?

3. more options from [specification](https://www.dpg.unipd.it/sites/dpg.unipd.it/files/Brainvision_Recorder.pdf)
