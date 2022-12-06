use std::collections::HashSet;

fn find_n_unique(input: &str, n: usize) -> String {
    for char_vec in input.chars().collect::<Vec<char>>().windows(n) {
        let mut unique_set: HashSet<char> = HashSet::new();
        unique_set.extend(char_vec);

        if unique_set.len() == n {
            let search_string: String = char_vec.into_iter().collect();
            return search_string;
        }
    }
    String::from("")
}

fn find_subset_index_w_offset(search_string: &str, search_subset: String, n: usize) -> usize {
    search_string.find(&search_subset).unwrap() + n
}

pub fn solve(input: &str) {
    let part_one = find_subset_index_w_offset(input, find_n_unique(input, 4), 4);
    let part_two = find_subset_index_w_offset(input, find_n_unique(input, 14), 14);
    println!("{} | {}", part_one, part_two);
}