fn main() {
    let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice2 = &s[..];
    println!("{}, {}", slice, slice2);
    println!("{}", s);
}
