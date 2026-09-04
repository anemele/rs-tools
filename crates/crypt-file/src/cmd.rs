use crate::io::replace_file_head;
use crate::key::random_key;
use crate::name::NameError;
use crate::name::generate_encrypt_name;
use crate::name::parse_encrypt_name;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum CryptError {
    #[error("{0}")]
    Name(#[from] NameError),

    #[error("{0}")]
    IO(#[from] std::io::Error),
}

pub struct CryptNamePair {
    pub old_name: String,
    pub new_name: String,
}

impl std::fmt::Display for CryptNamePair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[OK] {} => {}", self.old_name, self.new_name)
    }
}

type CryptResult = Result<CryptNamePair, CryptError>;

pub fn decrypt(path: impl AsRef<Path>) -> CryptResult {
    let path = path.as_ref();

    let (ename, dname, key) = parse_encrypt_name(path)?;
    let dpath = path.with_file_name(&dname);

    // FIXME: if io error, if the file content changer?
    replace_file_head(path, key)?;
    fs::rename(path, dpath)?;

    Ok(CryptNamePair {
        old_name: ename,
        new_name: dname,
    })
}

pub fn encrypt(path: impl AsRef<Path>) -> CryptResult {
    let path = path.as_ref();
    let key = random_key();

    let (name, ename) = generate_encrypt_name(path, &key)?;
    let epath = path.with_file_name(&ename);

    replace_file_head(path, key)?;
    fs::rename(path, epath)?;

    Ok(CryptNamePair {
        old_name: name,
        new_name: ename,
    })
}

pub fn glob_files(args: &[String]) -> impl Iterator<Item = PathBuf> + '_ {
    args.iter()
        .map(|p| glob::glob(p))
        .filter_map(|p| p.ok())
        .flatten()
        .filter_map(|p| p.ok())
        .filter(|p| p.is_file())
}
