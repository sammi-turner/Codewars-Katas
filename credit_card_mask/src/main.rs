/*
Usually when you buy something, you're asked whether your credit card number, phone number or
answer to your most secret question is still correct. However, since someone could look over your
shoulder, you don't want that shown on your screen. Instead, we mask it.

Your task is to write a function maskify, which changes all but the last four characters into '#'.

Examples :

"4556364607935616" --> "############5616"
     "64607935616" -->      "#######5616"
               "1" -->                "1"
                "" -->                 ""

"What was the name of your first pet?"

"Skippy" --> "##ippy"
*/

fn maskify(cc: &str) -> String {
    let mut ch;
    let mut result = String::new();
    let len:usize = cc.chars().count();
    let text = cc.to_string();

    let mut count:usize = 0;
    while (len - count) > 4 {
        result.push('#');
        count += 1;
    }
    while (len - count) > 0 {
        ch = text.chars().nth(count).unwrap();
        result.push(ch);
        count += 1;
    }
    return result;
}

fn main() {
    let result_str = maskify("4556364607935616");
    println!("{}", result_str);
}
