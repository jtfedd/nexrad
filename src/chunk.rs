//!
//! Struct definitions for chunk meta, encoded binary data, and decoded data structures.
//!

use std::fmt::Debug;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Metadata to identify a particular NEXRAD WSR-88D radar chunk file. A meta is specific to a
/// particular radar site, date, and identifier.
#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
pub struct ChunkMeta {
    site: String,
    date: NaiveDate,
    identifier: String,
}

impl ChunkMeta {
    pub(crate) fn new(site: String, date: NaiveDate, identifier: String) -> Self {
        Self { site, date, identifier }
    }

    /// The radar site this chunk was produced at, e.g. KDMX.
    pub fn site(&self) -> &String {
        &self.site
    }

    /// The date this chunk's data was collected on.
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    /// The unique identifier for this chunk.
    pub fn identifier(&self) -> &String {
        &self.identifier
    }
}

/// An encoded (and possibly-compressed) NEXRAD WSR-88D chunk file including sweep data. If
/// compressed, the data is compressed using BZIP2. See
/// [decompress_chunk](crate::decompress::decompress_chunk) and
/// [decode_chunk](crate::decode::decode_chunk).
#[derive(Serialize, Deserialize)]
pub struct EncodedChunk {
    meta: ChunkMeta,
    data: Vec<u8>,
}

impl EncodedChunk {
    pub(crate) fn new(meta: ChunkMeta, data: Vec<u8>) -> Self {
        Self { meta, data }
    }

    /// The identifying metadata for this chunk.
    pub fn meta(&self) -> &ChunkMeta {
        &self.meta
    }

    /// The raw, encoded, and possibly-compressed data for this chunk.
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Determines whether this chunk is compressed. If compressed, it should be
    /// [decompressed](crate::decompress::decompress_chunk) before being
    /// [decoded](crate::decode::decode_chunk).
    pub fn compressed(&self) -> bool {
        self.data.len() >= 30 && &self.data[28..30] == b"BZ"
    }
}

/// A decoded NEXRAD WSR-88D chunk file including sweep data.
#[derive(Serialize, Deserialize)]
pub struct Chunk {
    meta: ChunkMeta,
    file_header: FileHeader,
}

impl Chunk {
    pub(crate) fn new(meta: ChunkMeta, file_header: FileHeader) -> Self {
        Self { meta, file_header }
    }

    /// The identifying metadata for this chunk.
    pub fn meta(&self) -> &ChunkMeta {
        &self.meta
    }
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct FileHeader {
    /// Filename of the archive.
    pub filename: [u8; 9],

    /// File extension.
    pub ext: [u8; 3],

    /// Modified Julian date of the file.
    pub file_date: u32,

    /// Milliseconds of day since midnight of the file.
    pub file_time: u32,

    /// Unused field.
    pub unused1: [u8; 4],
}