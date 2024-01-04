use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bvreader::bv_reader::header::parser::{parse_header_version::parse_header_version, parse_dataorientation::DataOrientation, parse_binaryformat::BinaryFormat};
use bvreader::bv_reader::header::BVheader;
use bvreader::bv_reader::data::{BVData, parser::parse_bytestring::parse_bytestring_to_f32};
use bvreader::bv_reader::BVFile;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_header_version", |b| b.iter(|| parse_header_version(black_box("Brain Vision Data Exchange Header File Version 1.0"))));
    //c.bench_function("parse_header_full", |b| b.iter(|| BVheader::from_file(black_box("/home/projects/rusty/wip-bvreader/example/0001_Fake_Datensatz_mit_AttentionGetter_und_Pause-spo.vhdr"))));
    c.bench_function("parse_header_full", |b| b.iter(|| BVheader::from_file(black_box("src/bv_reader/data/testfiles/01_header.vhdr"))));
    c.bench_function("parse_bytestring", |b| b.iter(|| parse_bytestring_to_f32(black_box(vec![22,76,10,29,18,79,8,12]), black_box(false))));

    //

    let mut bvfile = BVFile::from_header("src/bv_reader/data/testfiles/01_header.vhdr").unwrap();
    let chaninfo = bvfile.bv_header.channel_info.clone();
    //let _ = bvfile.bv_data.scale_channels(bvfile.bv_header.channel_info).unwrap();

 
    let mut group = c.benchmark_group("full-parser");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(10);
    group.bench_function("parse_binary_data", |b| b.iter(|| BVData::from_file(black_box("src/bv_reader/data/testfiles/01_data.eeg"),black_box(71), black_box(BinaryFormat::IEEE_FLOAT_32),black_box(DataOrientation::MULTIPLEXED), black_box(false))));
    group.bench_function("parse_full_data", |b| b.iter(|| BVFile::from_header(black_box("src/bv_reader/data/testfiles/01_header.vhdr"))));
    group.bench_function("scale_channels", |b| b.iter(|| bvfile.bv_data.scale_channels(black_box(&chaninfo))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);