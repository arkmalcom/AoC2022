use std::collections::HashMap;


pub fn solve() {
    let contents: String = include_str!("../inputs/day2.txt")
                        .replace(" ", "")
                        .replace(" ", "\n")
                        .chars()
                        .collect();
    let mut part_one_total = 0;
    let mut part_two_total = 0;
                                

    for line in contents.lines() {
        let result = get_result(String::from(line));
        part_one_total += result.0;
        part_two_total += result.1;
    }
    println!("{}", part_one_total);
    println!("{}", part_two_total);
}

fn get_result(key: String) -> (i32, i32) {
    let mut outcomes = HashMap::new();

    outcomes.insert(String::from("AY"), (8, 4));
    outcomes.insert(String::from("AX"), (4, 3));
    outcomes.insert(String::from("AZ"), (3, 8));
    outcomes.insert(String::from("BY"), (5, 5));
    outcomes.insert(String::from("BX"), (1, 1));
    outcomes.insert(String::from("BZ"), (9, 9));
    outcomes.insert(String::from("CY"), (2, 6));
    outcomes.insert(String::from("CX"), (7, 2));
    outcomes.insert(String::from("CZ"), (6, 7));

    let (result_one, result_two) = outcomes[&key];
    (result_one, result_two)
}