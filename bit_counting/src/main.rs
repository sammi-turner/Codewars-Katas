/*
Write a function that takes an integer as input, and returns the number of bits that are equal to
one in the binary representation of that number. You can guarantee that input is non-negative.

Examples :
1234 is 10011010010 in binary, so the function should return 5.
65 is 1000001 in binary, so the function should return 2.
*/

fn count_bits(n: i64) -> u32 {
    let binary_rep = format!("{:b}", n);
    let max:usize = binary_rep.chars().count();

    let mut i:usize = 0;
    let mut bits: u32 = 0;
    let mut ch:char;

    while i < max {
        ch = binary_rep.chars().nth(i).unwrap();
        if ch == '1' {
            bits += 1;
        }
        i += 1;
    }

    return bits;
}

fn main() {
    let a = count_bits(1234);
    let b = count_bits(65);
    println!("{}", a);
    println!("{}", b);
}
