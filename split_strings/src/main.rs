/*
Complete the solution so that it splits the string into pairs of two characters.
If the string contains an odd number of characters then it should replace the missing second
character of the final pair with an underscore ('_').

Examples:

* 'abc' =>  ['ab', 'c_']
* 'abcdef' => ['ab', 'cd', 'ef']

*/

fn solution(s: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut len:usize = s.chars().count();
    let mut text = s.to_string();

    if len % 2 == 1 {
        text.push('_');
        len += 1;
    }

    len /= 2;
    let mut count:usize = 0;
    while count < len {
        let start: String = text.chars().take(2).collect();
        result.push(start);
        text = text.chars().skip(2).collect();
        count += 1;
    }

    return result;
}

fn main() {
    let my_str = "abcdefg";
    println!("{:?}", solution(my_str));
}
