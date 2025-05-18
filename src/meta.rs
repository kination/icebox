use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceMeta {
    // type of source(parquet, mysql, ...)
    pub source_type: String,
    pub version: String,
    pub db_name: String,
    pub collections: Vec<String>,
    pub timestamp: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub identifier: String,
    pub file_type: String,
    pub org_size: u64,
    pub compression: String,
    pub offset: u64,
    pub checksum: String,
    pub db_meta: Option<SourceMeta>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArchiveIndex {
    pub entries: Vec<FileEntry>
}
