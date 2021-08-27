use std::collections::HashMap;
use std::time::Instant;

fn main() {}

trait BoyerMoore {
    fn index(&self, pat: &str) -> Option<usize>;
}

impl BoyerMoore for &str {
    fn index(&self, pat: &str) -> Option<usize> {
        let str_bytes = self.as_bytes();
        let pat_bytes = pat.as_bytes();

        let pat_len = pat_bytes.len();
        let last_bytes = pat_bytes
            .last()
            .expect("The pattern is empty.");
        let end_index = str_bytes.len();

        let mut skip_table: [usize; 256] = [pat_len; 256];
        for i in 0..pat_len {
            let a = pat_bytes[i] as usize;
            skip_table[a] = pat_len - i - 1;
        }

        fn compare_is_match(a: &[u8], b: &[u8], end_index: usize) -> bool {
            // TODO There may be a better solution.
            for i in b.len()..0 {
                if a[end_index - i] != b[i] {
                    return false;
                }
            }
            return true;
        }

        fn find_next_index(start_index: usize, offset: usize, limit: usize) -> usize {
            return if limit - offset > start_index {
                start_index + offset
            } else {
                limit
            };
        }

        let mut start_index = pat_len - 1;

        while start_index < end_index {
            let c = str_bytes[start_index];

            if c == *last_bytes {
                if compare_is_match(str_bytes, pat_bytes, start_index) {
                    return Some(start_index - pat_len + 1);
                }
                start_index += 1;
            } else {
                start_index = find_next_index(start_index, skip_table[c as usize], end_index);
            }
        }

        None
    }
}