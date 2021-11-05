use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SYMBOL_ENCODINGS: HashMap<&'static str, (u32, u32)> = {
        let mut m = HashMap::new();
        m.insert(".",   (1, 1));
        m.insert("-",   (1, 3));
        m.insert("",    (0, 1));
        m.insert(" ",   (0, 3));
        m.insert("   ", (0, 7));
        m
    };    
}

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

    for v in sequences.values_mut() {
        v.sort();
        v.dedup();
    }

    sequences
}

fn detect_samplerate(code: &Vec<u32>) -> u32 {
    let mut single_unit_seqs: HashMap<u32, Vec<u32>> = HashMap::new();
    for &(s, l) in SYMBOL_ENCODINGS.values() {
        single_unit_seqs.entry(s).or_insert(vec![]).push(l);
    }
    for v in single_unit_seqs.values_mut() {
        v.sort();
    }

    let sequences = find_sequences(code);

    println!("{:?}, {:?}", sequences, single_unit_seqs);

    for (s, l) in sequences {
        let single_unit_l = &single_unit_seqs[&s];
        let a: Vec<_> = l
            .iter()
            .map(|n| {
                single_unit_l
                    .iter()
                    .filter(move |&single_unit_n| n % single_unit_n == 0)
                    .map(move |single_unit_n| n / single_unit_n)
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect();
        println!("{} - {:?} / {:?}", s, l, single_unit_l);
        println!("{:?}", a);
    }
    0
}

fn split_alternating(code: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut last_char: Option<u32> = None;
    let mut res = Vec::new();

    for &c in code {
        if Some(c) != last_char {
            res.push(vec![]);
            last_char = Some(c);
        }
        res.last_mut().unwrap().push(c);
    }

    res
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
        assert_eq!(seqs.get(&0u32), Some(&vec![1u32, 3u32]));
        assert_eq!(seqs.get(&1u32), Some(&vec![2u32, 4u32]));
    }

    #[test]
    fn find_sequences_no_duplicates() {
        let seqs = find_sequences(&preprocess_code("11101110"));
        assert_eq!(seqs.len(), 2);
        assert_eq!(seqs.get(&0u32), Some(&vec![1u32]));
        assert_eq!(seqs.get(&1u32), Some(&vec![3u32]));
    }

    #[test]
    #[ignore]
    fn detect_samplerate_with_single_char() {
        assert_eq!(detect_samplerate(&preprocess_code("10111")), 1);
        assert_eq!(detect_samplerate(&preprocess_code("1100111111")), 2);
        assert_eq!(detect_samplerate(&preprocess_code("111000111111111")), 3);
    }

    #[test]
    fn split_alternating_single_char() {
        assert_eq!(split_alternating(&preprocess_code("111")), vec![vec![1; 3]]);
        assert_eq!(split_alternating(&preprocess_code("0")), vec![vec![0]]);
    }

    #[test]
    fn split_alternating_multiple_char() {
        assert_eq!(split_alternating(&preprocess_code("1110")), vec![vec![1; 3], vec![0]]);
        assert_eq!(split_alternating(&preprocess_code("00100011")), vec![vec![0; 2], vec![1], vec![0; 3], vec![1; 2]]);
    }
}
