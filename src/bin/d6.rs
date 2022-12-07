fn different(i: &[u8]) -> bool {
    let mut mask: u32 = 0;
    for &c in i {
        let bit = 1 << (c - b'a');
        if mask & bit != 0 {
            return false;
        }
        mask |= bit;
    }
    return true;
}
fn main() {
    let input = include_bytes!("../../input/d6.txt");
    for (i, w) in input.windows(14).enumerate() {
        if different(w) {
            println!("{}: {:?}", i+14, w);
            break
        }
    }
}
