/*
The rgb function is incomplete. Complete it so that passing in RGB decimal values will result in a hexadecimal representation being
returned. Valid decimal values for RGB are 0 - 255. Any values that fall out of that range must be rounded to the closest valid
value.

Note: Your answer should always be 6 characters long, the shorthand with 3 will not work here.

The following are examples of expected output values:

rgb(255, 255, 255) -- returns FFFFFF
rgb(255, 255, 300) -- returns FFFFFF
rgb(0, 0, 0) -- returns 000000
rgb(148, 0, 211) -- returns 9400D3

*/

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!(
        "{:02X}{:02X}{:02X}",
        r.clamp(0, 255),
        g.clamp(0, 255),
        b.clamp(0, 255)
    )
}

fn main() {
    println!("{}", rgb(0, 0, 0));
    println!("{}", rgb(1, 2, 3));
    println!("{}", rgb(255, 255, 255));
    println!("{}", rgb(254, 253, 252));
    println!("{}", rgb(-20, 275, 125));
}
