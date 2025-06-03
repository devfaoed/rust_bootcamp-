// rust program to check for even and odd number from 0 to 100 
// using the three type of loop in rust

// making use of for loop
fn main(){
    
    println!("This is 4 for Loop");
    even_odd_for();
    
    println!("This is for while Loop");
    even_odd_while();

    println!("This is for Loop Loop");
    even_odd_loop_loop();
}

fn even_odd_for(){
    for i in 1..=100 {
        if i % 2 == 0 {
            println!("even number from 1 to 100 include, {}", i);
        }
        else {
            println!("odd number from 1 to 100 include, {}", i);
            
        }
    }
}


// making use of while loop
fn even_odd_while(){
    let mut num = 0;
    
    while num <= 100 {
        if num % 2 == 0 {
            println!("even number from 1 to 100 include, {}", num);
        }
        else{
            println!("odd number from 1 to 100 include, {}", num);
        }
        num += 1;
    } 
}

// making use of loop loop
fn even_odd_loop_loop(){
    let mut i = 0;
    loop{
        i+= 1;
        if i <= 100{
            if i % 2 == 0 {
            println!("even number from 1 to 100 include, {}", i);
            }
                else {
                    println!("odd number from 1 to 100 include, {}", i);
                }
        }
    }
}