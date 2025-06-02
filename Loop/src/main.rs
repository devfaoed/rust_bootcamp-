// the for loop
fn main(){
    for i in 1..=5{
    println!("the numbers are {}", i);
    }

    th();
    Wwhile();
    LLopp();
}

// testing from number 0 to 100 using for loop
fn th(){
    for num in 0..1000{
        println!("numbers are:");
        println!("{}", num)
    }
}

// the whie loop

fn Wwhile(){
    let mut counter = 0;
    while counter <= 100{
        println!("Counter {}", counter);
        counter += 1;
    }
}

// the loop loop 
fn LLopp(){
    let mut num = 0;
    loop {
        println!("number {} is present in the loop", num);
            num += 1;
            if(num > 500){
            break;
        }
    }
}