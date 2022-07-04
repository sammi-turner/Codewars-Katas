/*
You probably know the "like" system from Facebook and other pages. People can "like" blog posts,
pictures or other items. We want to create the text that should be displayed next to such an item.

Implement the function which takes an array containing the names of people that like an item.
It must return the display text as shown in the examples:

[]                                -->  "no one likes this"
["Peter"]                         -->  "Peter likes this"
["Jacob", "Alex"]                 -->  "Jacob and Alex like this"
["Max", "John", "Mark"]           -->  "Max, John and Mark like this"
["Alex", "Jacob", "Mark", "Max"]  -->  "Alex, Jacob and 2 others like this"

Note: For 4 or more names, the number in "and 2 others" simply increases.
*/

fn likes(names: &[&str]) -> String {
    let result:String;
    let size = names.len();
    if size > 3 {
        result = format!("{}, {} and {} others like this", names[0], names[1], (size - 2));
    }
    else if size == 3 {
        result = format!("{}, {} and {} like this", names[0], names[1], names[2]);
    }
    else if size == 2 {
        result = format!("{} and {} like this", names[0], names[1]);
    }
    else if size == 1 {
        result = format!("{} likes this", names[0]);
    }
    else {
        result = format!("no one likes this");
    }
    return result;
}

fn main() {
    let n0 = likes(&[]);
    let n1 = likes(&["Peter"]);
    let n2 = likes(&["Jacob", "Alex"]);
    let n3 = likes(&["Max", "John", "Mark"]);
    let n4 = likes(&["Alex", "Jacob", "Mark", "Max"]);
    println!("{}\n{}\n{}\n{}\n{}", n0, n1, n2, n3, n4);
}
