use itertools::Itertools;

fn p(c: u8) -> u8 {
    if c >= b'a' {
        return c - b'a' + 1;
    }
    return c - b'A' + 27;
}
fn common_score(bag: &[u8]) -> u32 {
    // Split bag into 2 halves
    let (left, right) = bag.split_at(bag.len() / 2);
    // Find common items in each part
    left.into_iter()
        .filter(|&x| right.contains(x))
        .unique()
        .map(|&x| p(x) as u32)
        .sum()
}

fn main() {
    let input = include_bytes!("../../input/d3.txt");
    let p1: u32 = input
        .split(|c| *c == b'\n')
        .map(|bag| common_score(bag))
        .sum();
    let p2: u32 = input
        .split(|c| *c == b'\n')
        .chunks(3)
        .into_iter()
        .map(|group| {
            let bags: Vec<&[u8]> = group.collect();
            bags[0]
                .into_iter()
                .filter(|&x| bags[1].contains(x) && bags[2].contains(x))
                .unique()
                .map(|&x| p(x) as u32)
                .sum::<u32>()
        })
        .sum();
    println!("p1: {}\np2: {}", p1, p2);
}
