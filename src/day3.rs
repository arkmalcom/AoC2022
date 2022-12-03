use itertools::Itertools;

pub fn solve() {
    let contents: &str = include_str!("../inputs/day3.txt");
    let lines = contents.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let total_priority_value: usize = lines.iter()
                                        .map(|l| find_shared_char(&l[..l.len()/2], &l[l.len()/2..]))
                                        .map(|c| get_char_priority(c[0]))
                                        .sum();
    let total_badge_priority: usize = lines.iter()
                                        .tuples()
                                        .map(|(l1, l2, l3)| find_shared_char(l1, &find_shared_char(l2, l3)))
                                        .map(|c| get_char_priority(c[0]))
                                        .sum();

    
    println!("{}", total_priority_value);
    println!("{}", total_badge_priority);
}

fn find_shared_char(byte_vec_one: &[u8], byte_vec_two: &[u8]) -> Vec<u8> {
    byte_vec_one.iter().copied().filter(|b| byte_vec_two.contains(b)).collect()
}

fn get_char_priority(char: u8) -> usize {
    let alphabet = (b'a'..=b'z').chain(b'A'..=b'Z');

    let char_to_priority = alphabet.into_iter().position(|c| c == char).unwrap() + 1;
    char_to_priority
}