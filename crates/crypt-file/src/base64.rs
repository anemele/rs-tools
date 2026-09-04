/// A customized base64 encoding.
/// based on base64.rs
use base64::{alphabet, engine};
use rand::seq::SliceRandom;
use rand_chacha::rand_core::SeedableRng;
use sha2::Digest;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
pub(crate) const ENV_CF_KEY: &str = "CRYPT_FILE_KEY";

fn gen_alphabet() -> String {
    let Ok(key) = std::env::var(ENV_CF_KEY) else {
        return ALPHABET.to_string();
    };

    let seed = key.as_bytes();
    let mut hasher = sha2::Sha256::new();
    hasher.update(seed);
    let seed: [u8; 32] = hasher.finalize().into();

    let mut rng = rand_chacha::ChaCha8Rng::from_seed(seed);
    let mut tmp = ALPHABET.as_bytes().to_vec();
    tmp.shuffle(&mut rng);
    tmp.into_iter().map(|x| x as char).collect()
}

pub(crate) fn new_engine() -> engine::GeneralPurpose {
    let ab = match alphabet::Alphabet::new(&gen_alphabet()) {
        Ok(ab) => ab,
        Err(_) => alphabet::URL_SAFE,
    };
    let config = engine::GeneralPurposeConfig::new()
        .with_encode_padding(false)
        .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);
    engine::GeneralPurpose::new(&ab, config)
}

#[test]
fn test_new_engine() {
    use base64::Engine as _;
    let engine = new_engine();

    let s = "hello base64";
    let e = engine.encode(s);
    assert!(!e.ends_with("="));
    let d = engine.decode(e).unwrap();

    assert_eq!(d, s.as_bytes())
}
