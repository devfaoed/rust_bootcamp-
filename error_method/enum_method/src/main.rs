fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    // let numbers: Vec<i32> = vec![];
    let first = numbers.first();

    match first{
        Some(value) => println!("First Element: {value}"),
        None => println!("List is empty"),
    }
}
