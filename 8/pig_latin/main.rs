fn pig_latin(word: &str) -> String {
    let first_char = word.chars().next();
    match first_char {
        Some(c) if c.is_alphabetic() && "aeiouAEIOU".contains(c) => format!("{}-hay", word),
        Some(c) if c.is_alphabetic() => format!("{}-{}ay", &word[c.len_utf8()..], c),
        _ => word.to_string(),
    }
}

fn to_pig_latin(s: &str) -> String {
    s.split_whitespace()
        .map(pig_latin)
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let input = "first apple";
    let output = to_pig_latin(input);
    println!("{}", output);
}
