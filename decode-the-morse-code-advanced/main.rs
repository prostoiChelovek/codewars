use std::collections::HashMap;

fn preprocess_code(code: &str) -> Vec<u32> {
    code.chars().flat_map(|x| x.to_digit(10)).collect()
}

fn find_sequences(code: &Vec<u32>) -> HashMap<u32, Vec<u32>> {
    let mut sequences = HashMap::new();
    let mut last_char: Option<u32> = None;
    for &c in code {
        let seq = sequences.entry(c).or_insert(vec![]);
        if Some(c) != last_char {
            seq.push(0);
            last_char = Some(c);
        }
        *seq.last_mut().unwrap() += 1;
    }
    sequences
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_sequences_single_char_single_seq() {
        let seqs = find_sequences(&preprocess_code("000"));
        assert_eq!(seqs.len(), 1);
        assert_eq!(seqs.get(&0u32), Some(&vec![3u32]));
    }

    #[test]
    fn find_sequences_two_chars_single_seq() {
        let seqs = find_sequences(&preprocess_code("00011"));
        assert_eq!(seqs.len(), 2);
        assert_eq!(seqs.get(&0u32), Some(&vec![3u32]));
        assert_eq!(seqs.get(&1u32), Some(&vec![2u32]));
    }

    #[test]
    fn find_sequences_two_chars_two_seqs() {
        let seqs = find_sequences(&preprocess_code("0001101111"));
        assert_eq!(seqs.len(), 2);
        assert_eq!(seqs.get(&0u32), Some(&vec![3u32, 1u32]));
        assert_eq!(seqs.get(&1u32), Some(&vec![2u32, 4u32]));
    }
}
