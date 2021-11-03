mod preloaded;
use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

fn decode_morse(encoded: &str) -> String {
    encoded
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
