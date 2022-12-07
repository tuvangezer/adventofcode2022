// Stack building funciton
fn build_stack_from_input(config: &str) -> Vec<Vec<u8>> {
    // Create a vector of 9 stacks
    let mut stacks = vec![vec![]; 9];
    // Iterate over the config in reverse
    for line in config.lines().rev().skip(1) {
        // Get every 4th char
        for (i, char) in line.chars().skip(1).step_by(4).enumerate() {
            if char.is_ascii_alphabetic() {
                // Push the char to the stack
                stacks[i].push(char as u8);
            }
        }
    }
    return stacks;
}
fn parse_op_string(input: &str) -> (usize, usize, usize) {
    // Split the input string on the space character
    let parts: Vec<&str> = input.split(' ').collect();

    // Parse the three numbers from the string
    let num1 = parts[1].parse::<usize>().unwrap();
    let num2 = parts[3].parse::<usize>().unwrap();
    let num3 = parts[5].parse::<usize>().unwrap();

    // Return the three numbers as a tuple
    (num1, num2 - 1, num3 - 1)
}

fn main() {
    // Read input
    let input = include_str!("../../input/d5.txt");
    // Split input at double newline
    let (config, ops) = input.split_at(input.find("\n\n").unwrap());
    // Build stack from config
    let mut stacks = build_stack_from_input(config);
    // Iterate over the operations
    for op in ops.lines().skip(2) {
        let (count, from, to) = parse_op_string(op);
        let move_amount = std::cmp::min(stacks[from].len(), count);
        // Part 1
        // for _ in 0..move_amount {
        //     let char = stacks[from].pop().unwrap();
        //     stacks[to].push(char);
        // }
        // Part 2
        // Get a slice of last move_size elements of stacks[from]
        let slice = &stacks[from][stacks[from].len() - move_amount..].to_vec();
        for char in slice {
            stacks[to].push(*char);
        }
        for _ in 0..move_amount {
            stacks[from].pop();
        }
    }
    // Print top of each stack
    for stack in stacks {
        print!("{}", *stack.last().unwrap_or(&b'0') as char);
    }
}
