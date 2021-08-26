use std::collections::HashMap;

fn main() {
    let s = "Hello, World";
    let s = "🐶🐔🐷🐮🐱";
    let pattern = "World";
    let pattern = "🐮";
    let pattern_chars = pattern.chars().collect::<Vec<_>>();

    let skip_table: HashMap<char, usize> = pattern
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            return (c, i);
        })
        .collect();

    let last_char = pattern_chars
        .last()
        .expect("There is a las char.");

    let p = pattern_chars.len();
    let mut start_index = p - 1;
    let string_chars = s.chars().collect::<Vec<char>>();
    let end_index = string_chars.len();

    while  start_index < end_index {
        let c = string_chars[start_index];


        if c == *last_char {
            if &string_chars[start_index-p+1..start_index+1] == pattern_chars {
                println!("{:?}", start_index - p + 1);
            }

            start_index += 1;
        } else {
            let &a = skip_table.get(&c).unwrap_or(&p);
            let i = a + start_index;
            if i < end_index {
                start_index = i;
            } else {
                //TODO not found.
                start_index = end_index;
            }
        }
    }
}

trait BoyerMoore {
    fn index(&self, pattern: &str) -> Option<usize>;
}