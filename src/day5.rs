use scan_fmt::scan_fmt;

pub fn solve(input: &str) {
    let crate_input = include_str!("../inputs/day5-crates.txt")
                            .lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    
    let mut stacks = create_n_stacks(&crate_input.last().unwrap(), crate_input.len() - 1);

    for crate_line in &crate_input[..crate_input.len()-1] {
        stacks = stack_crates(&mut stacks, crate_line).to_vec();
    }

    let mut stacks_9000 = stacks.clone();
    let mut stacks_9001 = stacks.clone();

    for line in input.lines() {
        let (qty, origin, dest) = scan_fmt!(line, "move {} from {} to {}", usize, usize, usize).unwrap();
        move_to_crate(&mut stacks_9000, qty, origin, dest);
        move_in_groups(&mut stacks_9001, qty, origin, dest);
    };

    println!("{:#?}", stacks_9000);
    println!("{:#?}", stacks_9001);
}

fn create_n_stacks(width: &[u8], height: usize) -> Vec<Vec<char>> {
    let stack_count = width.iter().copied().filter(|b| !b.is_ascii_whitespace()).count();
    let mut stacks = Vec::with_capacity(stack_count * height);
    for _ in 0..stack_count {
        stacks.push(vec![]);
    }
    stacks
}

fn stack_crates<'a>(stacks: &'a mut Vec<Vec<char>>, line: &'a [u8]) -> &'a mut Vec<Vec<char>> {
    let alpha_indices = line.iter().copied()
                        .enumerate()
                        .filter_map(|(index, b)| b.is_ascii_alphabetic().then(|| (index / 4, b as char)))
                        .collect::<Vec<_>>();
    
    for (i, v) in alpha_indices {
        stacks[i].insert(0, v);
    }
    stacks
}

fn move_to_crate(stacks: &mut Vec<Vec<char>>, qty: usize, origin: usize, dest: usize) {
    for _ in 0..qty {
        let crate_to_move = stacks[origin - 1].pop().unwrap();
        stacks[dest - 1].push(crate_to_move);
    }
}

fn move_in_groups(stacks: &mut Vec<Vec<char>>, qty: usize, origin: usize, dest: usize) {
    let length = stacks[origin-1].len();
    let mut crates_to_move = stacks[origin-1].drain(length-qty..).collect::<Vec<char>>();
    stacks[dest-1].append(&mut crates_to_move);
}