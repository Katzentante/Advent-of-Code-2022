fn main() {
    let input = std::fs::read_to_string("./src/input6.txt").unwrap();
    let signature = input.trim_end();

    let mut out = 0;
    let mut cache = [' '; 14];
    for (i, c) in signature.chars().enumerate() {
        cache.rotate_left(1);
        cache[13] = c;

        if is_unique(&cache) {
            out = i + 1;
            break;
        }
    }
    println!("{}", signature);
    println!("{:#?}\nPart 1: {}", cache, out);
}

fn is_unique(cache: &[char; 14]) -> bool {
    if cache[0] == ' ' {
        return false;
    }
    for i in 1..14 {
        for o in 0..i {
            if cache[i] == cache[o] {
                return false;
            }
        }
    }
    return true;
}
