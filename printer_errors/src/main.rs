/*

In a factory a printer prints labels for boxes. For one kind of boxes the printer has to use colors
which, for the sake of simplicity, are named with letters from a to m.

The colors used by the printer are recorded in a control string. For example a "good" control
string would be aaabbbbhaijjjm meaning that the printer used three times color a, four times color
b, one time color h then one time color a...

Sometimes there are problems: lack of colors, technical malfunction and a "bad" control string is
produced e.g. aaaxbbbbyyhwawiwjjjwwm with letters not from a to m.

You have to write a function printer_error which given a string will return the error rate of the
printer as a string representing a rational whose numerator is the number of errors and the
denominator the length of the control string. Don't reduce this fraction to a simpler expression.

The string has a length greater or equal to one and contains only letters from a to z.

Examples :

s1 = "aaabbbbhaijjjm"
printer_error(s) => "0/14"

s2 = "aaaxbbbbyyhwawiwjjjwwm"
printer_error(s) => "8/22"

*/

fn printer_error(s: &str) -> String {
    format!("{}/{}", s.chars().filter(err_char).count(), s.len())
}

fn err_char(c: &char) -> bool {
    match c {
        'a'..='m' => false,
        _ => true,
    }
}

fn main() {
    let s1 = printer_error("aaabbbbhaijjjm");
    let s2 = printer_error("aaaxbbbbyyhwawiwjjjwwm");
    println!("{}", s1);
    println!("{}", s2);
}
