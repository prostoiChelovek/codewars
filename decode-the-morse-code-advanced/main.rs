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
        .filter_map(|(ref c, ref n)| SYMBOL_ENCODINGS.get(c)?.get(n))
        .cloned()
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
    println!("{:?}", decode_bits("00001100111111000000000"));
    println!("{:?}", decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011"));
}

