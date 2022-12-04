pub fn solve(input: &str) {
    let part_one: u16 = input
                            .lines()
                            .map(|l| l.replace("-", ","))
                            .map(|l| to_i16_vector(l))
                            .map(|v| ranges_contain(v))
                            .sum();
    let part_two: u16 = input
                            .lines()
                            .map(|l| l.replace("-", ","))
                            .map(|l| to_i16_vector(l))
                            .map(|v| ranges_overlap(v))
                            .sum();
    println!("{}", part_one);
    println!("{}", part_two);
}

fn to_i16_vector(line: String) -> Vec<u16> {
    line.split(",").map(|i| i.parse::<u16>().unwrap()).collect()
}

fn ranges_contain(v: Vec<u16>) -> u16 {
    let [x1, x2, y1, y2] = [v[0], v[1], v[2], v[3]];
    match v {
        _ if (x1 <= y1 && x2 >= y2) => 1,
        _ if (y1 <= x1 && y2 >= x2) => 1,
        _ => 0,
    }
}

fn ranges_overlap(v: Vec<u16>) -> u16 {
    let [x1, x2, y1, y2] = [v[0], v[1], v[2], v[3]];
    match v {
        _ if (x1 <= y2 && y1 <= x2) => 1,
        _ => 0,
    }
}