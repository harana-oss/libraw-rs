extern crate libraw;

use std::fs;
use std::path::Path;

use libraw::Reader;

#[test]
fn it_can_read_processed_data() {
    let buf = fs::read(Path::new("tests/data/RAW_NIKON_D1.NEF")).expect("read in");

    let reader = Reader::new();
    let image = reader.read_8bit(&buf).expect("processing successful");

    assert_eq!(2012, image.width());
}