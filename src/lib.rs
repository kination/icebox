pub mod meta;
pub mod compression;
pub mod container;

pub use container::ArchiveBuilder;
pub use meta::{ArchiveIndex, FileEntry, SourceMeta};
pub use compression::compress_data;


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
