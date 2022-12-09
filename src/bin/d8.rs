fn main() {
    let input = include_bytes!("../../input/d8.txt");
    let map = input
        .split(|c| *c == b'\n')
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<u8>>>();
    // Record width and height
    let width = map[0].len();
    let height = map.len();
    // Record visible trees
    let mut visible_trees = 0;
    // Iterate over each member of the map
    for x in 0..height {
        'elements: for y in 0..width {
            // Check if edge of map
            if x == height - 1 || y == width - 1 || x == 0 || y == 0 {
                visible_trees += 1;
                continue 'elements;
            }
            let val = map[x][y];
            // Check from left
            for yl in 0..y {
                if map[x][yl] >= val {
                    break;
                }
                if yl == y - 1 {
                    visible_trees += 1;
                    continue 'elements;
                }
            }
            // Check from right
            for yr in (y + 1..width).rev() {
                if map[x][yr] >= val {
                    break;
                }
                if yr == y + 1 {
                    visible_trees += 1;
                    continue 'elements;
                }
            }
            // Check from top
            for xt in 0..x {
                if map[xt][y] >= val {
                    break;
                }
                if xt == x - 1 {
                    visible_trees += 1;
                    continue 'elements;
                }
            }
            // Check from bottom
            for xb in (x + 1..height).rev() {
                if map[xb][y] >= val {
                    break;
                }
                if xb == x + 1 {
                    visible_trees += 1;
                    continue 'elements;
                }
            }
        }
    }
    println!("{:?}", visible_trees);
    let mut max_score = 0;
    // Iterate over each member of the map
    for x in 1..height - 1 {
        for y in 1..width - 1 {
            let mut score = 1;
            let mut vis = 0;
            // Check to left
            for yl in (0..y).rev() {
                //println!("checking: {:?}, val: {}", map[x][yl] - b'0', map[x][y] - b'0');
                vis += 1;
                if map[x][yl] >= map[x][y] {
                    break;
                }
            }
            score *= vis;
            vis = 0;
            // Check to right
            for yr in y + 1..width {
                vis += 1;
                if map[x][yr] >= map[x][y] {
                    break;
                }
            }
            score *= vis;
            vis = 0;
            // Check to top
            for xt in (0..x).rev() {
                vis += 1;
                if map[xt][y] >= map[x][y] {
                    break;
                }
            }
            score *= vis;
            vis = 0;
            // Check to bottom
            for xb in x + 1..height {
                vis += 1;
                if map[xb][y] >= map[x][y] {
                    break;
                }
            }
            score *= vis;
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("{:?}", max_score);
}
