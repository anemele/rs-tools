use std::io::Read;

const COEFFICIENT: [u8; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
const LAST_NUMBER: &[u8] = b"10X987654321";

type VResult<T> = Result<T, String>;

fn calc_check_code(id_17: &str) -> VResult<char> {
    if id_17.len() != 17 {
        return Err(format!("not length 17: {}", id_17));
    }

    let mut sum = 0;
    for (i, c) in id_17.as_bytes().iter().enumerate() {
        if !c.is_ascii_digit() {
            return Err(format!("illegal char: {}", c));
        }
        sum += COEFFICIENT[i] * (c - 48);
    }

    let res = LAST_NUMBER[sum as usize % 11];
    Ok(res as char)
}

fn main() {
    let mut buf = String::new();
    match std::io::stdin().lock().read_to_string(&mut buf) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }

    for line in buf.lines() {
        let code = calc_check_code(line.trim()).unwrap_or_default();
        println!("{}", code);
    }
}
