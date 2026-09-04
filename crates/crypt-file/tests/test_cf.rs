use crypt_file::cmd::{decrypt, encrypt};
use sha256::try_digest;

// #[ignore = "to fix"]
#[test]
fn test_samples() {
    let paths = glob::glob("tests/samples/*").unwrap();
    for path in paths {
        let path = path.unwrap();

        let old_hash = try_digest(&path).unwrap();

        let en_name = encrypt(&path).unwrap().new_name;
        let new_path = path.with_file_name(en_name);

        let de_name = decrypt(new_path).unwrap().new_name;
        let init_name = path.file_name().unwrap().to_str().unwrap();

        assert_eq!(de_name, init_name);
        assert!(path.exists());

        let new_hash = try_digest(&path).unwrap();
        assert_eq!(old_hash, new_hash);
    }
}
