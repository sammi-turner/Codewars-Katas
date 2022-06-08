/*
In this kata you are required to, given a string, replace every letter with its position in the
alphabet.

If anything in the text isn't a letter, ignore it and don't return it.

Example :

alphabet_position("The sunset sets at twelve o' clock.") =>
"20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11"
*/

fn letter_position(ch:char) -> u32 {
    let mut result:u32 = 0;
    let is_lower:bool = ch.is_ascii_lowercase();
    let is_upper:bool = ch.is_ascii_uppercase();

    if is_lower {
        result = ch as u32;
        result -= 96;
    } else if is_upper {
        result = ch as u32;
        result -= 64;
    }
    return result;
}

fn alphabet_position(text: &str) -> String {
    let mut ch;
    let mut pos:u32;
    let mut result = String::new();

    let length:usize = text.chars().count();
    let mut count:usize = 0;

    while count < length {
        ch = text.chars().nth(count).unwrap();
        pos = letter_position(ch);
        if pos > 0 {
            result.push_str(&format!("{} ", pos));
        }
        count += 1;
    }

    result.pop();
    return result;
}

fn main() {
    let ans = alphabet_position("The sunset sets at twelve o' clock.");
    println!("{}", ans);
}
