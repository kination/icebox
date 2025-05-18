
use crate::meta::{SourceMeta, FileEntry, ArchiveIndex};

use serde_json;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const MAGIC_KEY: &[u8] = b"ICEBOX";


pub struct ArchiveBuilder {
    pub index: ArchiveIndex,
    pub data: Vec<u8>
}

impl ArchiveBuilder {
    pub fn new() -> Self {
        ArchiveBuilder {
            index: ArchiveIndex {
                entries: Vec::new()
            },
            data: Vec::new()
        }
    }

    pub fn add_file(
        &mut self,
        identifier: &str,
        file_type: &str,
        file_data: &[u8],
        compression: String,
        db_meta: Option<SourceMeta>
    ) {
        let offset: u64 = self.data.len() as u64;
        let org_size: u64 = file_data.len() as u64;

        let entry = FileEntry {
            identifier: identifier.to_string(),
            file_type: file_type.to_string(),
            org_size: org_size,
            compression: compression,
            offset: offset,
            checksum: "".to_string(),
            db_meta: db_meta
        };

        self.index.entries.push(entry);
        self.data.extend_from_slice(file_data);
    }

    pub fn write_to_file<P: AsRef<Path>>(
        &self,
        output_path: P,
        compression_level: i32
    ) -> std::io::Result<()> {

        let compression_data = crate::compression::compress_data(&self.data, compression_level).expect("Compression Failed");
        let index_json = serde_json::to_vec(&self.index).expect("Serialization Failed");

        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        writer.write_all(MAGIC_KEY)?;

        let index_length: u64 = index_json.len() as u64;
        writer.write_all(&index_length.to_le_bytes())?;
        writer.write_all(&index_json)?;
        writer.write_all(&compression_data)?;

        writer.flush()?;
        Ok(())
    }
}
