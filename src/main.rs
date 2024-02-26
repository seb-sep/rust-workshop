mod enums;
mod iterators;
mod handling;

fn main() {
    let s1 = String::from("hello");
    let mut s2 = String::from("world");

    let len = calculate_length(&s1);
    change(&mut s2);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


