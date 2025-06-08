fn dev(x:f64, y:f64) -> Result<f64, &'static str>{
    if y == 0.0 {
        return Err("Error: cannot divide by zero");
    }
    Ok(x/y)
}

fn main(){
    let result : Result<f64, &str> = dev(10.0, 0.0);
    match result{
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Result: {}", err),
    }
}