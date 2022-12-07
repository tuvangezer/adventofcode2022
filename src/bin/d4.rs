fn main() {
    let input = include_str!("../../input/d4.txt");
    let mut count = 0;
    let _v = input
        .split("\n")
        .map(|pair| {
            let mut elves = pair.split(",");
            let range1 = elves
                .nth(0)
                .unwrap()
                .split("-")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let range2 = elves
                .nth(0)
                .unwrap()
                .split("-")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            if range1[0] <= range2[0] && range1[1] >= range2[0] {
                count += 1;
            } else if range2[0] <= range1[0] && range2[1] >= range1[0] {
                count += 1;
            }
        })
        .collect::<Vec<()>>();
    println!("{}", count);
}
