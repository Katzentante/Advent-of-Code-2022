fn main() {
    let input = std::fs::read_to_string("./src/input4.txt").unwrap();
    let lines = input.lines();

    let out: usize = lines
        .map(|line| -> usize {
            let mut ranges = line
                .split(',')
                .map(|sec| -> (usize, usize) {
                    sec.split_once('-')
                        .map(|num| {
                            (
                                num.0.parse::<usize>().unwrap(),
                                num.1.parse::<usize>().unwrap(),
                            )
                        })
                        .unwrap()
                })
                .collect::<Vec<(usize, usize)>>();

            // sort so that the one with the smaller bottom is the upper
            ranges.sort_by(|a, b| a.0.cmp(&b.0));
            let smaller = ranges.get(0).unwrap();
            let greater = ranges.get(1).unwrap();

            if greater.0 > smaller.1 {
                0
            } else {
                1
            }
        })
        .sum();

    println!("Part 2: {}", out);
}
