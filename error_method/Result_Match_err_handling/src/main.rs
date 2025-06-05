fn divide(a:i32, b:i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("cannot divide by zero")
    } else {
        Ok(a/b)
     }
} 

fn main() {
    let outcome: Result<i32, &str> = divide(10, 2);
    match outcome {
        Ok(result) => println!("Answer {}", result),
        Err(err) => println!("Error {}", err) 
    }
    let outcome2: Result<i32, &str> = divide(10, 0);
    match outcome2 {
        Ok(result) => println!("Answer {}", result),
        Err(err) => println!("Error {}", err) 
    }
}
