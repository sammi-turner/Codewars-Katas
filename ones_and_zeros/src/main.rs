/*
Given an array of ones and zeroes, convert the equivalent binary value to an integer.

Examples :

[0, 0, 0, 1] ==> 1
[0, 0, 1, 0] ==> 2
[0, 1, 0, 1] ==> 5
[1, 0, 0, 1] ==> 9
[0, 0, 1, 0] ==> 2
[0, 1, 1, 0] ==> 6
[1, 1, 1, 1] ==> 15
[1, 0, 1, 1] ==> 11

However, the arrays can have varying lengths, not just limited to 4.
*/

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    let size = slice.len();
    let mut count = 0;
    let mut total:u32 = 0;

    while count < size {
        if slice[size - count - 1] == 1 {
            total += 2_u32.pow(count as u32);
        }
        count += 1;
    }

    return total;
}

fn main() {
    let e1 = binary_slice_to_number(&[0, 0, 0, 1]);
    let e2 = binary_slice_to_number(&[0, 0, 1, 0]);
    let e3 = binary_slice_to_number(&[0, 1, 0, 1]);
    println!("{} {} {}", e1, e2, e3);
}
