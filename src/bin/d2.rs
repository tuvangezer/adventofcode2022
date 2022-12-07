use itertools::Itertools;

fn p1() {
    let file_str = include_str!("../../input/d2.txt");
    let mut score = 0;
    // Iterate over chars, 4 at a time
    let iter = file_str.chars().chunks(4);
    for mut group in &iter {
        let abc = group.nth(0).unwrap();
        let xyz = group.nth(1).unwrap();
        match xyz {
            'X' => score += 1,
            'Y' => score += 2,
            'Z' => score += 3,
            _ => (),
        }
        if (abc == 'A' && xyz == 'Y') || (abc == 'B' && xyz == 'Z') || (abc == 'C' && xyz == 'X') {
            score += 6;
        } else if (abc as i32 - 'A' as i32) == (xyz as i32 - 'X' as i32) {
            score += 3;
        }
    }
    println!("P1 Score: {}", score);
}
fn p2() {
    let file_str = include_str!("../../input/d2.txt");
    let mut score = 0;
    // Iterate over chars, 4 at a time
    let iter = file_str.chars().chunks(4);
    for mut group in &iter {
        let abc = group.nth(0).unwrap();
        let xyz = group.nth(1).unwrap();
        match xyz {
            'Y' => {
                score += 3;
                match abc {
                    'A' => score += 1,
                    'B' => score += 2,
                    'C' => score += 3,
                    _ => (),
                }
            }
            'Z' => {
                score += 6;
                match abc {
                    'A' => score += 2,
                    'B' => score += 3,
                    'C' => score += 1,
                    _ => (),
                }
            }
            'X' => match abc {
                'A' => score += 3,
                'B' => score += 1,
                'C' => score += 2,
                _ => (),
            },
            _ => (),
        }
    }
    println!("P2 Score: {}", score);
}
fn main() {
    p1();
    p2();
}
