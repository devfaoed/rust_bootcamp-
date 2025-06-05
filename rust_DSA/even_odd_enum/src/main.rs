fn is_even(num: i32) -> Option<bool> {
    if num % 2 == 0 {
        return Some(true);
    } else if num % 2 == 1 {
        return Some(false);
    } 
    None
}
fn main() {
   let number = -1;
   match is_even(number) {
        Some(true) => println!("The number {} given is even", number),
        Some(false) => println!("The number {} given is odd", number),
        None => println!("the number given is negative"),
    }
}
