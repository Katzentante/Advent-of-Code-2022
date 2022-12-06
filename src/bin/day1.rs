// see: https://adventofcode.com/2022/day/1
fn main() {
    let input = std::fs::read_to_string("./src/input1.txt").unwrap();
    let lines = input.split("\n\n");
    let mut lines: Vec<usize> = lines
        .map(|line| line.split("\n").flat_map(|num| num.parse::<usize>()).sum())
        .collect();
    lines.sort_by(|a, b| b.cmp(a));

    println!("Part1: {:?}", lines[0]);
    println!("Part2: {:?}", lines.iter().take(3).sum::<usize>());
}
