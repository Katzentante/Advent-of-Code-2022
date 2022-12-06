fn main() {
    let input = std::fs::read_to_string("./src/input5.txt").unwrap();
    let (stacks, calls): (&str, &str) = input.split_once("\n\n").unwrap();

    for s in stacks.lines() {
        println!("{}", s);
    }

    let len = stacks
        .lines()
        .enumerate()
        .skip_while(|(i, _s)| i != &(stacks.lines().count() - 1))
        .next()
        .unwrap()
        .1
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .len();

    let mut realstacks: Vec<Vec<char>> = vec![Vec::new(); len];
    let mut lines = stacks.lines();
    // remove the last line of the stacks
    lines.next_back();
    let tmp = lines.map(|st| st.chars().collect::<Vec<char>>());

    for i in tmp {
        let i = i.chunks(4);
        for (index, m) in i.enumerate() {
            if m[1] != ' ' {
                realstacks[index].push(m[1]);
            }
        }
    }

    // reverse the stacks as there were put in from top to bottom
    let mut realstacks: Vec<&mut Vec<char>> = realstacks.iter_mut().map(|r| {r.reverse();r}).collect();

    for call in calls.lines() {
        let call: Vec<usize> = call
            .split(' ')
            .filter_map(|st| st.parse::<usize>().ok())
            .collect();
        for _ in 0..call[0] {
            let boxer = realstacks[call[1] - 1].pop().unwrap();
            // print!("Moving {:?} from {} to {} -> ", boxer, call[1], call[2]);
            realstacks[call[2] - 1].push(boxer);
            // for r in realstacks.iter() {
            //     print!("{:?}, ", r)
            // }
            // println!();
        }
    }

    let mut out = String::new();
    for r in realstacks.iter_mut() {
        // println!("{:?}", r);
        out.push(r.pop().unwrap());
    }
    println!("{out}");
}
