mod base64;
pub mod cmd;
mod name;

mod key {
    use rand::RngExt;

    const KEY_SET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const KEY_LEN: usize = KEY_SET.len();
    const MIN_KEY_LEN: usize = 4;
    const MAX_KEY_LEN: usize = 8;

    pub fn random_key() -> Vec<u8> {
        let mut rng = rand::rng();

        let len = rng.random_range(MIN_KEY_LEN..MAX_KEY_LEN).min(MAX_KEY_LEN);
        (0..len)
            .map(|_| KEY_SET[rng.random_range(0..KEY_LEN)])
            .collect()
    }
}

mod io {
    use std::fs::OpenOptions;
    use std::io::{Read, Seek, Write};
    use std::path::Path;

    const BLOCK_SIZE: usize = 1 << 12;

    pub fn replace_file_head(path: impl AsRef<Path>, key: Vec<u8>) -> Result<(), std::io::Error> {
        let path = path.as_ref();

        let mut file = OpenOptions::new().read(true).write(true).open(path)?;

        let mut buf = [0u8; BLOCK_SIZE];
        let num = file.read(&mut buf)?;

        let buf = &mut buf[..num];
        buf.iter_mut()
            .zip(key.iter().cycle())
            .for_each(|(b, k)| *b ^= k);

        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(buf)?;

        Ok(())
    }
}
