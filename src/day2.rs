const VICTORY_PTS: u32 = 6;
const DRAW_PTS: u32 = 3;
const LOSS_PTS: u32 = 0;

pub fn solve() {
    let contents = include_str!("../inputs/day2.txt");
    let mapped_contents:String = contents
                                .replace(" ", "")
                                .chars()
                                .map(|x| match x {
                                    'A' | 'X' => '1',
                                    'B' | 'Y' => '2',
                                    'C' | 'Z' => '3',
                                    _ => ' ',
                                }).collect();
    let mapped_contents_pt_two:String = contents
                                        .replace(" ", "")
                                        .chars()
                                        .map(|x| match x {
                                            'A' => '1',
                                            'B' => '2',
                                            'C' => '3',
                                            'X' => '0',
                                            'Y' => '3',
                                            'Z' => '6',
                                            _ => ' ',
                                        }).collect();

    let outcome_pt_one = resolve_part_one(mapped_contents);
    let outcome_pt_two = resolve_part_two(mapped_contents_pt_two);

    println!("{}", outcome_pt_one);
    println!("{}", outcome_pt_two);
}

fn resolve_game(left: u32, right: u32) -> u32 {
    let outcome = match [left, right] {
        _ if left == right => right + DRAW_PTS,
        _ if left == 1 && right == 3 => right + LOSS_PTS,
        _ if left == 3 && right == 1 => right + VICTORY_PTS,
        _ if left > right => right + LOSS_PTS,
        _ => right + VICTORY_PTS,
    };
    outcome
}

fn resolve_part_one(mapped_contents: String) -> u32 {
    let mut total_outcome = 0;

    for line in mapped_contents.replace(" ", "\n").lines() {
        let left = line.chars().nth(0).expect("Character not found!").to_digit(10).unwrap();
        let right = line.chars().nth(1).expect("Character not found!").to_digit(10).unwrap();

        let outcome = resolve_game(left, right);
        total_outcome += outcome;
    }
    total_outcome
}

fn resolve_part_two(mapped_contents: String) -> u32 {
    let mut total_outcome = 0;

    for line in mapped_contents.replace(" ", "\n").lines() {
        let left = line.chars().nth(0).expect("Character not found!").to_digit(10).unwrap();
        let right = line.chars().nth(1).expect("Character not found!").to_digit(10).unwrap();

        let outcome = resolve_game_pt_two(left, right);
        total_outcome += outcome;
    }
    total_outcome
}

fn resolve_game_pt_two(left: u32, right: u32) -> u32 {
    let outcome = match [left, right] {
        _ if right == 3 => left + DRAW_PTS,
        _ if left == 3 && right == 6 => 1 + VICTORY_PTS,
        _ if left == 1 && right == 0 => 3 + LOSS_PTS,
        _ if right == 0 => left - 1 + LOSS_PTS,
        _ => left + 1 + VICTORY_PTS,
    };
    outcome
}