/*
Given a positive number n > 1 find the prime factor decomposition of n. The result will be a string.

Example:
86240 => "(2**5)(5)(7**2)(11)"

*/

use std::collections::BTreeMap;

fn prime_factors(n: i64) -> String {
    let mut result = BTreeMap::new();
    let mut prime = 2;
    let mut test = n;
    while prime <= test {
        if test % prime == 0 {
            test /= prime;
            let counter = result.entry(prime).or_insert(0);
            *counter += 1;
        } else {
            prime += 1;
        }
    }
    let mut output = String::from("");
    for (key, val) in result.iter() {
        if *val != 1 {
            output.push_str(format!("({}**{})", key, val).as_str());
        } else {
            output.push_str(format!("({})", key).as_str());
        }
    }
    return output;
}

fn main() {
    println!("{}\n{}\n{}", prime_factors(1234), prime_factors(67), prime_factors(88));
}

