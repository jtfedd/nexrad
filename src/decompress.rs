//!
//! TODO
//!

use crate::result::Result;
use bzip2::read::BzDecoder;
use std::io::Read;
use crate::model::Archive2Header;

pub struct Archive2File {
    pub header: Archive2Header,
    pub records: Vec<MessageHeader>,
}

fn decompress_and_decode_archive2_file<R: Read>(reader: &mut R) -> Result<>

fn decompress_ldm_record<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
    let mut record_size = [0; 4];
    reader.read_exact(&mut record_size)?;
    let record_size = u32::from_be_bytes(record_size);

    let compressed_data = reader.take(record_size as u64);

    let mut decompressed_data = Vec::new();
    BzDecoder::new(compressed_data).read_to_end(&mut decompressed_data)?;

    Ok(decompressed_data)
}
