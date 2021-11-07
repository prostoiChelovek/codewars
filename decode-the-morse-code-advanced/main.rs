use itertools::Itertools;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SYMBOL_ENCODINGS: HashMap<char, HashMap<usize, &'static str>> = {
        let mut m1 = HashMap::new();
        m1.insert(1, ".");
        m1.insert(3, "-");

        let mut m0 = HashMap::new();
        m0.insert(1, "");
        m0.insert(3, " ");
        m0.insert(7, "   ");

        let mut m = HashMap::new();
        m.insert('0', m0);
        m.insert('1', m1);

        m
    };    
}

lazy_static! {
    static ref MORSE_CODE: HashMap<String, String> = {
        vec![
            (".-", "A"), ("-...", "B"), ("-.-.", "C"),
            ("-..", "D"), (".", "E"), ("..-.", "F"),
            ("--.", "G"), ("....", "H"), ("..", "I"),
            (".---", "J"), ("-.-", "K"), (".-..", "L"),
            ("--", "M"), ("-.", "N"), ("---", "O"),
            (".--.", "P"), ("--.-", "Q"), (".-.", "R"),
            ("...", "S"), ("-", "T"), ("..-", "U"),
            ("...-", "V"), (".--", "W"), ("-..-", "X"),
            ("-.--", "Y"), ("--..", "Z"),
            (".----", "1"), ("..---", "2"), ("...--", "3"),
            ("....-", "4"), (".....", "5"), ("-....", "6"),
            ("--...", "7"), ("---..", "8"), ("----.", "9"), ("-----", "0"),
            ("--..--", ", "), (".-.-.-", "."), ("..--..", "?"),
            ("-..-.", "/"), ("-....-", "-"), ("-.--.", "("), ("-.--.-", ")"),
        ].into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
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

/*
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
*/

fn split_alternating<T>(code: &Vec<T>) -> Vec<&[T]>
where
    T: std::cmp::PartialEq + Copy,
{
    let mut last_char: Option<T> = None;
    let mut last_end: usize = 0;
    let mut res: Vec<_> = Vec::new();

    for (i, c) in code.iter().cloned().map(Some).chain(std::iter::once(None)).enumerate() {
        if last_char.is_none() { last_char = c; }
        if c != last_char {
            res.push(&code[last_end..i]);
            last_end = i;
            last_char = c;
        }
    }

    res
}

fn fold_sequences<T>(seqs: Vec<&[T]>) -> Vec<(T, usize)>
where
    T: std::cmp::PartialEq + Copy,
{
    seqs
        .iter()
        .map(|seq| {
            seq
                .iter()
                .fold((None, 0usize), |acc, x| (Some(*x), acc.1 + 1))
        })
        .map(|x| (x.0.unwrap(), x.1))
        .collect()
}

pub fn decode_bits(encoded: &str) -> String {
    let it = encoded
        .chars()
        .map(|c| (c, 1))
        .coalesce(|a, b| {
            if a.0 == b.0 { Ok((a.0, a.1 + b.1)) }
            else { Err((a, b)) }
        });
    let samplerate = it
        .clone()
        .flat_map(|(c, n)| {
            SYMBOL_ENCODINGS[&c]
                .keys()
                .sorted()
                .rev()
                .filter(move |len| n % *len == 0)
                .map(move |len| n / len)
                .collect::<Vec<_>>()
        })
        .sorted()
        .fold(HashMap::<usize, usize>::new(), |mut m, x| {
            *m.entry(x).or_default() += 1;
            m
        })
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .unwrap();
    it
        .map(|(c, n)| (c, n / samplerate))
        .map(|(ref c, ref n)| SYMBOL_ENCODINGS[c][n])
        .collect::<String>()
        .split("   ")
        .map(|x| {
            x.split(' ')
                .map(|s| MORSE_CODE.get(s).unwrap_or(&String::from("")).clone())
                .collect::<String>()
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    println!("{:?}", decode_bits("10111"));
    println!("{:?}", decode_bits("1100111111"));
    println!("{:?}", decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011"));
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

    #[test]
    fn fold_sequences_empty() {
        assert_eq!(fold_sequences(split_alternating(&preprocess_code(""))), vec![]);
    }

    #[test]
    fn fold_sequences_single_char() {
        assert_eq!(fold_sequences(split_alternating(&preprocess_code("111"))), vec![(1, 3)]);
        assert_eq!(fold_sequences(split_alternating(&preprocess_code("0"))), vec![(0, 1)]);
    }

    #[test]
    fn fold_sequences_multiple_char() {
        assert_eq!(fold_sequences(split_alternating(&preprocess_code("1110"))), vec![(1, 3), (0, 1)]);
        assert_eq!(fold_sequences(split_alternating(&preprocess_code("00100011"))), vec![(0, 2), (1, 1), (0, 3), (1, 2)]);
    }
}
