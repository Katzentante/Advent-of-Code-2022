fn main() {
    let input = std::fs::read_to_string("./src/input3.txt").unwrap();
    let out: usize = input
        .lines()
        .map(|line| {
            let length = line.len();
            let (first, last) = line.split_at(length / 2);
            let mut m: usize = 0;
            for c in first.chars() {
                if last.contains(c) {
                    m = as_num(c)
                }
            }
            m
        })
        .sum();

    println!("Part 1: {:?}", out);
    // for c in "abcde ABCD".chars() {
    //     println!("{} : {}", c, as_num(c));
    // }
}

fn as_num(c: char) -> usize {
    if c.is_lowercase() {
        c as usize - 96
    } else if c.is_uppercase() {
        c as usize - 38
    } else {
        0
    }
}
