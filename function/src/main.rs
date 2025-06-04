// function to add multiply numbe togetr
fn add(a:i32, b:i32) -> i32 {
    a + b
}

fn main(){
    let result = add(4, 5);
    println!("the final ans is {}", result);
    let advanced_result = mul(result, 8);
    println!("advanced result is {}", advanced_result);
    combine();
}


// function to multiply two number
fn mul(a:i32, b:i32) -> i32{
    a * b
}


// another form 
fn both_add_mul(a:i32, b:i32) -> (i32, i32){
    (a + b, a * b)
}

fn combine () {
    let result: (i32, i32) = both_add_mul(7, 9);
    println!("addition = {}", result.0);
    println!("multiplication = {}", result.1)
}