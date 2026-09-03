use numsys::NumSys;

fn main() {
    let sys = NumSys::default();
    println!("{}", sys.convert("100", 2, 10).unwrap()); // 4
    println!("{}", sys.convert("101010", 2, 10).unwrap()); // 42
    println!("{}", sys.convert("101010", 2, 16).unwrap()); // 2A
    println!("{}", sys.convert("2A", 16, 2).unwrap()); // 101010
}
