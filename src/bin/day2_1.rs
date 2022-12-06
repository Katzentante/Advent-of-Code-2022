#![feature(str_split_whitespace_as_str)]
fn main() {
    let input = std::fs::read_to_string("./src/input2.txt").unwrap();
    let lines = input.lines();
    let lines = lines.map(|x| {
        (
            to_num(&x.chars().nth(0).unwrap_or_default()),
            to_num(&x.chars().nth(2).unwrap_or_default()),
        )
    });
    let mut total = 0;
    let mut total2 = 0;
    for (e, y) in lines.clone() {
        total += y;
        if e == y {
            total += 3;
            continue;
        }
        if (e == 1 && y == 2) || (e == 2 && y == 3) || (e == 3 && y == 1) {
            total += 6;
            continue;
        }
        // else if enemy wins total isnt incremented
        // => it can be wegelassenwerden
    }
    println!("Part 1: {}", total);

    // Part2
    for (e, y) in lines {

    }
}

fn to_num(s: &char) -> usize {
    match s {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}
