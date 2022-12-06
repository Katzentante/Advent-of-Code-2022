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
    for (e, y) in lines.clone() {
        // X: loose
        // Y: draw
        // Z: win

        total += match y {
            // loose
            1 => match e {
                1 => 3,
                2 => 1,
                3 => 2,
                _ => 0,
            },
            // draw
            2 => e + 3,
            // win
            3 => match e {
                1 => 2+6,
                2 => 3+6,
                3 => 1+6,
                _ => 0,
            },
            _ => 0,
        }
    }
    println!("Part 2: {}", total);
}

fn to_num(s: &char) -> usize {
    match s {
        'A' => 1, // Rock
        'B' => 2, // Paper
        'C' => 3, // Scissors
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}
