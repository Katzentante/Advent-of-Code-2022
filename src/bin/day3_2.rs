fn main() {
    let input = std::fs::read_to_string("./src/input3.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    let mut total = 0;
    'group: for group in lines.chunks(3) {
        // find common chars in all three group members
        let mut commons = Vec::new();
        for c in group[0].chars() {
            if group[1].contains(c) {
                commons.push(c);
            }
        }

        for (i, c) in commons.iter().enumerate() {
            // if !group[2].contains(c) {
            //     commons.remove(i);
            // }
            if group[2].contains(*c) {
                total += as_num(*c);
                continue 'group;
            }
        }
    }

    println!("Part 2: {}", total);
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
