use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input/d1.txt")?;
    let reader = BufReader::new(file);

    let mut cur_sum = 0;
    let mut seen = BinaryHeap::new();
    for line in reader.lines() {
        let line = line?;
        if line.len() > 0 {
            let num = line.parse::<i32>().unwrap();
            cur_sum += num;
        } else {
            seen.push(cur_sum);
            cur_sum = 0;
        }
    }
    seen.push(cur_sum);
    // Sum top 3 values
    let mut sum = 0;
    for _ in 0..3 {
        sum += seen.pop().unwrap();
    }
    println!("{}", sum);
    Ok(())
}
