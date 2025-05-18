use std::io::Write;
use zstd::stream::write::Encoder;


pub fn compress_data(data: &[u8], level: i32) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder =  Encoder::new(Vec::new(), level)?;
    encoder.write_all(data)?;
    encoder.finish()
}
