fn main() {
    let input = std::fs::read_to_string("./src/input6.txt").unwrap();
    let signal = input.trim_end();

    let mut out = 0;
    let mut cache = [' '; 4];
    for (i, c) in signal.chars().enumerate() {
        cache.rotate_left(1);
        cache[3] = c;

        if is_unique(&cache) {
            out = i + 1;
            break;
        }
    }
    println!("{}", signal);
    println!("{:#?}\nPart 1: {}", cache, out);
}

fn is_unique(cache: &[char; 4]) -> bool {
    if cache[0] == ' ' {
        return false;
    }
    if cache[1] == cache[0] {
        return false;
    }
    if cache[2] == cache[1] || cache[2] == cache[0] {
        return false;
    }
    if cache[3] == cache[2] || cache[3] == cache[1] || cache[3] == cache[0] {
        return false;
    }
    return true;
}
