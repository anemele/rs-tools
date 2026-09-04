use crate::base64::new_engine;
use base64::{DecodeError, Engine};
use std::{path::Path, str::Utf8Error};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum NameError {
    #[error("failed to get file name")]
    BaseName,

    #[error("name missing dot: {0}")]
    MissingDot(String),

    #[error("name starts with dot: {0}")]
    StartsWithDot(String),

    #[error("base64 decode error")]
    B64Decode(#[from] DecodeError),

    #[error("utf8 decode error")]
    UTF8Decode(#[from] Utf8Error),
}

type RResult<T> = Result<T, NameError>;

#[inline]
fn get_file_name(path: &Path) -> Option<&str> {
    path.file_name()?.to_str()
}

/// 根据原文件名和密钥生成加密文件名
///
/// 返回：原文件名，加密文件名
pub fn generate_encrypt_name(path: impl AsRef<Path>, key: &[u8]) -> RResult<(String, String)> {
    let path = path.as_ref();

    let name = get_file_name(path).ok_or(NameError::BaseName)?;

    let bytes_ret: Vec<u8> = name
        .as_bytes()
        .iter()
        .zip(key.iter().cycle())
        .map(|(b, k)| b ^ k)
        .collect();

    let mut ret = new_engine().encode(bytes_ret);
    ret.push('.');
    let suffix = std::str::from_utf8(key)?;
    ret.push_str(suffix);

    Ok((name.to_string(), ret))
}

/// 根据加密文件名，解析出原文件名和密钥
///
/// 某文件名 `foo.bar` 其中 `bar` 为密钥，`foo` 为加密文件名，
/// 它们都是 base64 编码，需要解码后才能得到原文件名和密钥。
///
/// 返回：加密文件名，解密文件名（原文件名），解密后的密钥
pub fn parse_encrypt_name(path: impl AsRef<Path>) -> RResult<(String, String, Vec<u8>)> {
    let path = path.as_ref();

    let name = get_file_name(path).ok_or(NameError::BaseName)?;

    let (ename, key) = name
        .rsplit_once(".")
        .ok_or(NameError::MissingDot(name.to_string()))?;
    if ename.is_empty() {
        return Err(NameError::StartsWithDot(name.to_string()));
    }

    let key = key.as_bytes();
    let bytes_ename: Vec<u8> = new_engine()
        .decode(ename)?
        .iter()
        .zip(key.iter().cycle())
        .map(|(b, k)| b ^ k)
        .collect();

    let dname = std::str::from_utf8(&bytes_ename)?;

    let ret = (name.to_string(), dname.to_string(), key.to_vec());
    Ok(ret)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::base64::ENV_CF_KEY;

    const KEY: &[u8] = b"key";
    const NAME: &str = "ABCDEFG";
    const EN_NAME: &str = "l9WLNEIKNI.key";

    #[test]
    fn test_generate_encrypt_name() {
        #[cfg(target_family = "windows")]
        let path = Path::new("C:\\Windows");
        #[cfg(target_family = "unix")]
        let path = Path::new("/home/unix");

        let path = path.join(NAME);

        let (name, ename) = temp_env::with_var(ENV_CF_KEY, Some(ENV_CF_KEY), || {
            generate_encrypt_name(path, KEY).unwrap()
        });

        assert_eq!(NAME, name);
        assert_eq!(EN_NAME, ename);
    }

    #[test]
    fn test_parse_encrypt_name() {
        #[cfg(target_family = "windows")]
        let root_path = Path::new("C:\\Windows");
        #[cfg(target_family = "unix")]
        let root_path = Path::new("/home/unix");

        let path = root_path.join(EN_NAME);

        unsafe {
            std::env::set_var(ENV_CF_KEY, ENV_CF_KEY);
        }

        let (dname, ename, key) = temp_env::with_var(ENV_CF_KEY, Some(ENV_CF_KEY), || {
            parse_encrypt_name(path).unwrap()
        });

        assert_eq!(dname, EN_NAME);
        assert_eq!(ename, NAME);
        assert_eq!(key, KEY);

        let path = root_path.join(".gitignore");
        let ret = parse_encrypt_name(path);
        assert!(ret.is_err());
    }

    #[test]
    fn test_with_invalid_key() {
        #[cfg(target_family = "windows")]
        let root_path = Path::new("C:\\Windows");
        #[cfg(target_family = "unix")]
        let root_path = Path::new("/home/unix");

        let path = root_path.join(NAME);
        let (_, ename) = temp_env::with_var(ENV_CF_KEY, Some("valid_key"), || {
            generate_encrypt_name(path, KEY).unwrap()
        });

        let path = root_path.join(ename);
        temp_env::with_var(ENV_CF_KEY, Some("invalid_key"), || {
            let ret = parse_encrypt_name(path);
            assert!(ret.is_err());
        });
    }
}
