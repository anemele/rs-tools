use numsys::NumSys;

fn main() {
    let ns = NumSys::new("+-*/^%$#").unwrap();
    println!("{}", ns.convert("#", 8, 2).unwrap()); // ---
}
